use anyhow::{Context, Result};
use crossterm::{
    QueueableCommand,
    style::{
        Attribute, Attributes, Color, ResetColor, SetAttribute, SetAttributes, SetForegroundColor,
    },
    terminal,
};
use std::{
    io::{self, Read, StdoutLock, Write},
    sync::mpsc::{Sender, SyncSender, sync_channel},
    thread,
};

use crate::{
    app_state::{AppState, ExercisesProgress},
    clear_terminal,
    exercise::{OUTPUT_CAPACITY, RunnableExercise, solution_link_line},
    term::progress_bar,
};

use super::{InputPauseGuard, WatchEvent, terminal_event::terminal_event_handler};

const HEADING_ATTRIBUTES: Attributes = Attributes::none()
    .with(Attribute::Bold)
    .with(Attribute::Underlined);

#[derive(PartialEq, Eq)]
enum DoneStatus {
    DoneWithSolution(String),
    DoneWithoutSolution,
    Pending,
}

pub struct WatchState<'a> {
    app_state: &'a mut AppState,
    output: Vec<u8>,
    show_hint: bool,
    done_status: DoneStatus,
    manual_run: bool,
    term_width: u16,
    terminal_event_unpause_sender: SyncSender<()>,
}

impl<'a> WatchState<'a> {
    pub fn build(
        app_state: &'a mut AppState,
        watch_event_sender: Sender<WatchEvent>,
        manual_run: bool,
    ) -> Result<Self> {
        let term_width = terminal::size()
            .context("Failed to get the terminal size")?
            .0;

        let (terminal_event_unpause_sender, terminal_event_unpause_receiver) = sync_channel(0);

        thread::Builder::new()
            .spawn(move || {
                terminal_event_handler(
                    watch_event_sender,
                    terminal_event_unpause_receiver,
                    manual_run,
                )
            })
            .context("Failed to spawn a thread to handle terminal events")?;

        Ok(Self {
            app_state,
            output: Vec::with_capacity(OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            manual_run,
            term_width,
            terminal_event_unpause_sender,
        })
    }

    pub fn run_current_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        // Ignore any input until running the exercise is done.
        let _input_pause_guard = InputPauseGuard::scoped_pause();

        self.show_hint = false;

        writeln!(
            stdout,
            "\nChecking the exercise `{}`. Please wait…",
            self.app_state.current_exercise().name,
        )?;

        let success = self
            .app_state
            .current_exercise()
            .run_exercise(Some(&mut self.output), self.app_state.cmd_runner())?;
        self.output.push(b'\n');
        if success {
            self.done_status =
                if let Some(solution_path) = self.app_state.current_solution_path()? {
                    DoneStatus::DoneWithSolution(solution_path)
                } else {
                    DoneStatus::DoneWithoutSolution
                };
        } else {
            self.app_state
                .set_pending(self.app_state.current_exercise_ind())?;

            self.done_status = DoneStatus::Pending;
        }

        self.render(stdout)?;
        Ok(())
    }

    pub fn reset_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        clear_terminal(stdout)?;

        stdout.write_all(b"Resetting will undo all your changes to the file ")?;
        stdout.write_all(self.app_state.current_exercise().path.as_bytes())?;
        stdout.write_all(b"\nReset (y/n)? ")?;
        stdout.flush()?;

        {
            let mut stdin = io::stdin().lock();
            let mut answer = [0];
            loop {
                stdin
                    .read_exact(&mut answer)
                    .context("Failed to read the user's input")?;

                match answer[0] {
                    b'y' | b'Y' => {
                        self.app_state.reset_current_exercise()?;

                        // The file watcher reruns the exercise otherwise.
                        if self.manual_run {
                            self.run_current_exercise(stdout)?;
                        }
                    }
                    b'n' | b'N' => self.render(stdout)?,
                    _ => continue,
                }

                break;
            }
        }

        self.terminal_event_unpause_sender.send(())?;

        Ok(())
    }

    pub fn handle_file_change(
        &mut self,
        exercise_ind: usize,
        stdout: &mut StdoutLock,
    ) -> Result<()> {
        if self.app_state.current_exercise_ind() != exercise_ind {
            return Ok(());
        }

        self.run_current_exercise(stdout)
    }

    /// Move on to the next exercise if the current one is done.
    pub fn next_exercise(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        match self.done_status {
            DoneStatus::DoneWithSolution(_) | DoneStatus::DoneWithoutSolution => (),
            DoneStatus::Pending => return Ok(ExercisesProgress::CurrentPending),
        }

        self.app_state.done_current_exercise::<true>(stdout)
    }

    fn show_prompt(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.done_status != DoneStatus::Pending {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(b"n")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b":")?;
            stdout.queue(SetAttribute(Attribute::Underlined))?;
            stdout.write_all(b"next")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b" / ")?;
        }

        let mut show_key = |key, postfix| {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(&[key])?;
            stdout.queue(ResetColor)?;
            stdout.write_all(postfix)
        };

        if self.manual_run {
            show_key(b'r', b":run / ")?;
        }

        if !self.show_hint {
            show_key(b'h', b":hint / ")?;
        }

        show_key(b'l', b":list / ")?;
        show_key(b'c', b":check all / ")?;
        show_key(b'x', b":reset / ")?;
        show_key(b'q', b":quit ? ")?;

        stdout.flush()
    }

    pub fn render(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        // Prevent having the first line shifted if clearing wasn't successful.
        stdout.write_all(b"\n")?;
        clear_terminal(stdout)?;

        stdout.write_all(&self.output)?;

        if self.show_hint {
            stdout
                .queue(SetAttributes(HEADING_ATTRIBUTES))?
                .queue(SetForegroundColor(Color::Cyan))?;
            stdout.write_all(b"Hint")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n")?;

            stdout.write_all(self.app_state.current_exercise().hint.as_bytes())?;
            stdout.write_all(b"\n\n")?;
        }

        if self.done_status != DoneStatus::Pending {
            stdout
                .queue(SetAttribute(Attribute::Bold))?
                .queue(SetForegroundColor(Color::Green))?;
            stdout.write_all("Exercise done ✓".as_bytes())?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n")?;

            if let DoneStatus::DoneWithSolution(solution_path) = &self.done_status {
                solution_link_line(stdout, solution_path, self.app_state.emit_file_links())?;
            }

            stdout.write_all(
                "When done experimenting, enter `n` to move on to the next exercise 🦀\n\n"
                    .as_bytes(),
            )?;
        }

        progress_bar(
            stdout,
            self.app_state.n_done(),
            self.app_state.exercises().len() as u16,
            self.term_width,
        )?;

        stdout.write_all(b"\nCurrent exercise: ")?;
        self.app_state
            .current_exercise()
            .terminal_file_link(stdout, self.app_state.emit_file_links())?;
        stdout.write_all(b"\n\n")?;

        self.show_prompt(stdout)?;

        Ok(())
    }

    pub fn show_hint(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        if !self.show_hint {
            self.show_hint = true;
            self.render(stdout)?;
        }

        Ok(())
    }

    pub fn check_all_exercises(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        // Ignore any input until checking all exercises is done.
        let _input_pause_guard = InputPauseGuard::scoped_pause();

        if let Some(first_pending_exercise_ind) = self.app_state.check_all_exercises(stdout)? {
            // Only change exercise if the current one is done.
            if self.app_state.current_exercise().done {
                self.app_state
                    .set_current_exercise_ind(first_pending_exercise_ind)?;
                Ok(ExercisesProgress::NewPending)
            } else {
                Ok(ExercisesProgress::CurrentPending)
            }
        } else {
            self.app_state.render_final_message(stdout)?;
            Ok(ExercisesProgress::AllDone)
        }
    }

    pub fn update_term_width(&mut self, width: u16, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.term_width != width {
            self.term_width = width;
            self.render(stdout)?;
        }

        Ok(())
    }
}
