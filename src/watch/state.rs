use anyhow::{Context, Result};
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
    term::progress_bar,
};

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
}

impl<'a> WatchState<'a> {
    pub fn build(app_state: &'a mut AppState, manual_run: bool) -> Result<Self> {
        let term_width = terminal::size()
            .context("Failed to get the terminal size")?
            .0;

        Ok(Self {
            app_state,
            output: Vec::with_capacity(OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            manual_run,
            term_width,
        })
    }

    pub fn run_current_exercise(&mut self, stdout: &mut StdoutLock) -> Result<()> {
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

    pub fn handle_file_change(
        &mut self,
        exercise_ind: usize,
        stdout: &mut StdoutLock,
    ) -> Result<()> {
        // Don't skip exercises on file changes to avoid confusion from missing exercises.
        // Skipping exercises must be explicit in the interactive list.
        // But going back to an earlier exercise on file change is fine.
        if self.app_state.current_exercise_ind() < exercise_ind {
            return Ok(());
        }

        self.app_state.set_current_exercise_ind(exercise_ind)?;
        self.run_current_exercise(stdout)
    }

    /// Move on to the next exercise if the current one is done.
    pub fn next_exercise(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        if self.done_status == DoneStatus::Pending {
            return Ok(ExercisesProgress::CurrentPending);
        }

        self.app_state.done_current_exercise(stdout)
    }

    fn show_prompt(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.manual_run {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(b"r")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b":run / ")?;
        }

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

        if !self.show_hint {
            stdout.queue(SetAttribute(Attribute::Bold))?;
            stdout.write_all(b"h")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b":hint / ")?;
        }

        stdout.queue(SetAttribute(Attribute::Bold))?;
        stdout.write_all(b"l")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b":list / ")?;

        stdout.queue(SetAttribute(Attribute::Bold))?;
        stdout.write_all(b"q")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b":quit ? ")?;

        stdout.flush()
    }

    pub fn render(&self, stdout: &mut StdoutLock) -> io::Result<()> {
        // Prevent having the first line shifted if clearing wasn't successful.
        stdout.write_all(b"\n")?;
        clear_terminal(stdout)?;

        stdout.write_all(&self.output)?;

        if self.show_hint {
            stdout
                .queue(SetAttributes(
                    Attributes::from(Attribute::Bold).with(Attribute::Underlined),
                ))?
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
                solution_link_line(stdout, solution_path)?;
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
            .terminal_file_link(stdout)?;
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

    pub fn update_term_width(&mut self, width: u16, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.term_width != width {
            self.term_width = width;
            self.render(stdout)?;
        }

        Ok(())
    }
}
