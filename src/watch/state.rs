use anyhow::Result;
use crossterm::{
    style::{Attribute, ContentStyle, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use std::{
    fmt::Write as _,
    io::{self, StdoutLock, Write as _},
    sync::mpsc::Receiver,
    time::Duration,
};

use crate::{
    exercise::{Exercise, State},
    state_file::StateFile,
};

pub struct WatchState<'a> {
    writer: StdoutLock<'a>,
    rx: Receiver<DebounceEventResult>,
    exercises: &'a [Exercise],
    exercise: &'a Exercise,
    current_exercise_ind: usize,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    message: Option<String>,
    prompt: Vec<u8>,
}

impl<'a> WatchState<'a> {
    pub fn new(
        state_file: &StateFile,
        exercises: &'a [Exercise],
        rx: Receiver<DebounceEventResult>,
    ) -> Self {
        let current_exercise_ind = state_file.next_exercise_ind();
        let exercise = &exercises[current_exercise_ind];

        let writer = io::stdout().lock();

        let prompt = format!(
            "\n\n{}int/{}lear/{}uit? ",
            "h".bold(),
            "c".bold(),
            "q".bold()
        )
        .into_bytes();

        Self {
            writer,
            rx,
            exercises,
            exercise,
            current_exercise_ind,
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

        if !output.status.success() {
            self.stdout = Some(output.stdout);
            self.stderr = Some(output.stderr);
            return Ok(false);
        }

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

            self.stdout = Some(output.stdout);
            self.message = Some(message);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn try_recv_event(&mut self) -> Result<()> {
        let Ok(events) = self.rx.recv_timeout(Duration::from_millis(100)) else {
            return Ok(());
        };

        if let Some(current_exercise_ind) = events?
            .iter()
            .filter_map(|event| {
                if event.kind != DebouncedEventKind::Any
                    || !event.path.extension().is_some_and(|ext| ext == "rs")
                {
                    return None;
                }

                self.exercises
                    .iter()
                    .position(|exercise| event.path.ends_with(&exercise.path))
            })
            .min()
        {
            self.current_exercise_ind = current_exercise_ind;
        } else {
            return Ok(());
        };

        while self.current_exercise_ind < self.exercises.len() {
            self.exercise = &self.exercises[self.current_exercise_ind];
            if !self.run_exercise()? {
                break;
            }

            self.current_exercise_ind += 1;
        }

        Ok(())
    }

    pub fn show_prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(&self.prompt)?;
        self.writer.flush()
    }

    pub fn render(&mut self) -> io::Result<()> {
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

        self.show_prompt()
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
