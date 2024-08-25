use anyhow::Result;
use crossterm::{
    style::{
        Attribute, Attributes, Color, ResetColor, SetAttribute, SetAttributes, SetForegroundColor,
    },
    terminal, QueueableCommand,
};
use std::io::{self, StdoutLock, Write};

use crate::{
    app_state::{AppState, ExercisesProgress},
    clear_terminal,
    exercise::{solution_link_line, RunnableExercise, OUTPUT_CAPACITY},
    term::{progress_bar, terminal_file_link},
};

#[derive(PartialEq, Eq)]
enum DoneStatus {
    DoneWithSolution(String),
    DoneWithoutSolution,
    Pending,
}

pub struct WatchState<'a> {
    stdout: StdoutLock<'a>,
    app_state: &'a mut AppState,
    output: Vec<u8>,
    show_hint: bool,
    done_status: DoneStatus,
    manual_run: bool,
}

impl<'a> WatchState<'a> {
    pub fn new(app_state: &'a mut AppState, manual_run: bool) -> Self {
        // TODO: Take stdout as arg.
        let stdout = io::stdout().lock();

        Self {
            stdout,
            app_state,
            output: Vec::with_capacity(OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            manual_run,
        }
    }

    #[inline]
    pub fn into_writer(self) -> StdoutLock<'a> {
        self.stdout
    }

    pub fn run_current_exercise(&mut self) -> Result<()> {
        self.show_hint = false;

        writeln!(
            self.stdout,
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

        self.render()?;
        Ok(())
    }

    pub fn handle_file_change(&mut self, exercise_ind: usize) -> Result<()> {
        // Don't skip exercises on file changes to avoid confusion from missing exercises.
        // Skipping exercises must be explicit in the interactive list.
        // But going back to an earlier exercise on file change is fine.
        if self.app_state.current_exercise_ind() < exercise_ind {
            return Ok(());
        }

        self.app_state.set_current_exercise_ind(exercise_ind)?;
        self.run_current_exercise()
    }

    /// Move on to the next exercise if the current one is done.
    pub fn next_exercise(&mut self) -> Result<ExercisesProgress> {
        if self.done_status == DoneStatus::Pending {
            return Ok(ExercisesProgress::CurrentPending);
        }

        self.app_state.done_current_exercise(&mut self.stdout)
    }

    fn show_prompt(&mut self) -> io::Result<()> {
        if self.manual_run {
            self.stdout.queue(SetAttribute(Attribute::Bold))?;
            self.stdout.write_all(b"r")?;
            self.stdout.queue(ResetColor)?;
            self.stdout.write_all(b":run / ")?;
        }

        if self.done_status != DoneStatus::Pending {
            self.stdout.queue(SetAttribute(Attribute::Bold))?;
            self.stdout.write_all(b"n")?;
            self.stdout.queue(ResetColor)?;
            self.stdout.write_all(b":")?;
            self.stdout.queue(SetAttribute(Attribute::Underlined))?;
            self.stdout.write_all(b"next")?;
            self.stdout.queue(ResetColor)?;
            self.stdout.write_all(b" / ")?;
        }

        if !self.show_hint {
            self.stdout.queue(SetAttribute(Attribute::Bold))?;
            self.stdout.write_all(b"h")?;
            self.stdout.queue(ResetColor)?;
            self.stdout.write_all(b":hint / ")?;
        }

        self.stdout.queue(SetAttribute(Attribute::Bold))?;
        self.stdout.write_all(b"l")?;
        self.stdout.queue(ResetColor)?;
        self.stdout.write_all(b":list / ")?;

        self.stdout.queue(SetAttribute(Attribute::Bold))?;
        self.stdout.write_all(b"q")?;
        self.stdout.queue(ResetColor)?;
        self.stdout.write_all(b":quit ? ")?;

        self.stdout.flush()
    }

    pub fn render(&mut self) -> io::Result<()> {
        // Prevent having the first line shifted if clearing wasn't successful.
        self.stdout.write_all(b"\n")?;
        clear_terminal(&mut self.stdout)?;

        self.stdout.write_all(&self.output)?;

        if self.show_hint {
            self.stdout
                .queue(SetAttributes(
                    Attributes::from(Attribute::Bold).with(Attribute::Underlined),
                ))?
                .queue(SetForegroundColor(Color::Cyan))?;
            self.stdout.write_all(b"Hint\n")?;
            self.stdout.queue(ResetColor)?;

            self.stdout
                .write_all(self.app_state.current_exercise().hint.as_bytes())?;
            self.stdout.write_all(b"\n\n")?;
        }

        if self.done_status != DoneStatus::Pending {
            self.stdout
                .queue(SetAttribute(Attribute::Bold))?
                .queue(SetForegroundColor(Color::Green))?;
            self.stdout.write_all("Exercise done ✓\n".as_bytes())?;
            self.stdout.queue(ResetColor)?;

            if let DoneStatus::DoneWithSolution(solution_path) = &self.done_status {
                solution_link_line(&mut self.stdout, solution_path)?;
            }

            writeln!(
                self.stdout,
                "When done experimenting, enter `n` to move on to the next exercise 🦀\n",
            )?;
        }

        let line_width = terminal::size()?.0;
        progress_bar(
            &mut self.stdout,
            self.app_state.n_done(),
            self.app_state.exercises().len() as u16,
            line_width,
        )?;

        self.stdout.write_all(b"\nCurrent exercise: ")?;
        terminal_file_link(
            &mut self.stdout,
            self.app_state.current_exercise().path,
            Color::Blue,
        )?;
        self.stdout.write_all(b"\n\n")?;

        self.show_prompt()?;

        Ok(())
    }

    pub fn show_hint(&mut self) -> io::Result<()> {
        self.show_hint = true;
        self.render()
    }
}
