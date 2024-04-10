use anyhow::{Context, Result};
use crossterm::{
    style::{Attribute, ContentStyle, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    fmt::Write as _,
    io::{self, StdoutLock, Write as _},
};

use crate::{
    exercise::{Exercise, State},
    progress_bar::progress_bar,
    state_file::StateFile,
};

pub struct WatchState<'a> {
    writer: StdoutLock<'a>,
    exercises: &'static [Exercise],
    exercise: &'static Exercise,
    current_exercise_ind: usize,
    progress: u16,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    message: Option<String>,
    prompt: Vec<u8>,
}

impl<'a> WatchState<'a> {
    pub fn new(state_file: &StateFile, exercises: &'static [Exercise]) -> Self {
        let current_exercise_ind = state_file.next_exercise_ind();
        let progress = state_file.progress().iter().filter(|done| **done).count() as u16;
        let exercise = &exercises[current_exercise_ind];

        let writer = io::stdout().lock();

        let prompt = format!(
            "\n\n{}int/{}lear/{}ist/{}uit? ",
            "h".bold(),
            "c".bold(),
            "l".bold(),
            "q".bold(),
        )
        .into_bytes();

        Self {
            writer,
            exercises,
            exercise,
            current_exercise_ind,
            progress,
            stdout: None,
            stderr: None,
            message: None,
            prompt,
        }
    }

    #[inline]
    pub fn into_writer(self) -> StdoutLock<'a> {
        self.writer
    }

    pub fn run_exercise(&mut self) -> Result<bool> {
        let output = self.exercise.run()?;
        self.stdout = Some(output.stdout);

        if !output.status.success() {
            self.stderr = Some(output.stderr);
            return Ok(false);
        }

        self.stderr = None;

        if let State::Pending(context) = self.exercise.state()? {
            let mut message = format!(
                "
You can keep working on this exercise or jump into the next one by removing the {} comment:

",
                "`I AM NOT DONE`".bold(),
            );

            for context_line in context {
                let formatted_line = if context_line.important {
                    context_line.line.bold()
                } else {
                    context_line.line.stylize()
                };

                writeln!(
                    message,
                    "{:>2} {}  {}",
                    ContentStyle {
                        foreground_color: Some(crossterm::style::Color::Blue),
                        background_color: None,
                        underline_color: None,
                        attributes: Attribute::Bold.into()
                    }
                    .apply(context_line.number),
                    "|".blue(),
                    formatted_line,
                )?;
            }

            self.message = Some(message);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn run_exercise_with_ind(&mut self, exercise_ind: usize) -> Result<bool> {
        self.exercise = self
            .exercises
            .get(exercise_ind)
            .context("Invalid exercise index")?;
        self.current_exercise_ind = exercise_ind;

        self.run_exercise()
    }

    pub fn show_prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(&self.prompt)?;
        self.writer.flush()
    }

    pub fn render(&mut self) -> Result<()> {
        self.writer.execute(Clear(ClearType::All))?;

        if let Some(stdout) = &self.stdout {
            self.writer.write_all(stdout)?;
        }

        if let Some(stderr) = &self.stderr {
            self.writer.write_all(stderr)?;
        }

        if let Some(message) = &self.message {
            self.writer.write_all(message.as_bytes())?;
        }

        self.writer.write_all(b"\n")?;
        let line_width = size()?.0;
        let progress_bar = progress_bar(self.progress, self.exercises.len() as u16, line_width)?;
        self.writer.write_all(progress_bar.as_bytes())?;

        self.show_prompt()?;

        Ok(())
    }

    pub fn show_hint(&mut self) -> io::Result<()> {
        self.writer.write_all(self.exercise.hint.as_bytes())?;
        self.show_prompt()
    }

    pub fn handle_invalid_cmd(&mut self) -> io::Result<()> {
        self.writer.write_all(b"Invalid command")?;
        self.show_prompt()
    }
}
