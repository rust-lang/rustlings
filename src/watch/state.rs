use anyhow::Result;
use crossterm::{
    style::Stylize,
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, StdoutLock, Write},
    process::Output,
};

use crate::{
    app_state::{AppState, ExercisesProgress},
    progress_bar::progress_bar,
};

pub struct WatchState<'a> {
    writer: StdoutLock<'a>,
    app_state: &'a mut AppState,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    show_hint: bool,
    show_done: bool,
}

impl<'a> WatchState<'a> {
    pub fn new(app_state: &'a mut AppState) -> Self {
        let writer = io::stdout().lock();

        Self {
            writer,
            app_state,
            stdout: None,
            stderr: None,
            show_hint: false,
            show_done: false,
        }
    }

    #[inline]
    pub fn into_writer(self) -> StdoutLock<'a> {
        self.writer
    }

    pub fn run_current_exercise(&mut self) -> Result<()> {
        self.show_hint = false;

        let output = self.app_state.current_exercise().run()?;
        self.stdout = Some(output.stdout);

        if output.status.success() {
            self.stderr = None;
            self.show_done = true;
        } else {
            self.app_state
                .set_pending(self.app_state.current_exercise_ind())?;

            self.stderr = Some(output.stderr);
            self.show_done = false;
        }

        self.render()
    }

    pub fn run_exercise_with_ind(&mut self, exercise_ind: usize) -> Result<()> {
        self.app_state.set_current_exercise_ind(exercise_ind)?;
        self.run_current_exercise()
    }

    pub fn next_exercise(&mut self) -> Result<ExercisesProgress> {
        if !self.show_done {
            self.writer
                .write_all(b"The current exercise isn't done yet\n")?;
            self.show_prompt()?;
            return Ok(ExercisesProgress::Pending);
        }

        self.app_state.done_current_exercise(&mut self.writer)
    }

    fn show_prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(b"\n")?;

        if self.show_done {
            self.writer.write_fmt(format_args!("{}ext/", 'n'.bold()))?;
        }

        if !self.show_hint {
            self.writer.write_fmt(format_args!("{}int/", 'h'.bold()))?;
        }

        self.writer
            .write_fmt(format_args!("{}ist/{}uit? ", 'l'.bold(), 'q'.bold()))?;

        self.writer.flush()
    }

    pub fn render(&mut self) -> Result<()> {
        // Prevent having the first line shifted.
        self.writer.write_all(b"\n")?;

        self.writer.execute(Clear(ClearType::All))?;

        if let Some(stdout) = &self.stdout {
            self.writer.write_all(stdout)?;
            self.writer.write_all(b"\n")?;
        }

        if let Some(stderr) = &self.stderr {
            self.writer.write_all(stderr)?;
            self.writer.write_all(b"\n")?;
        }

        self.writer.write_all(b"\n")?;

        if self.show_hint {
            self.writer.write_fmt(format_args!(
                "{}\n{}\n\n",
                "Hint".bold().cyan().underlined(),
                self.app_state.current_exercise().hint,
            ))?;
        }

        if self.show_done {
            self.writer.write_fmt(format_args!(
                "{}\n\n",
                "Exercise done ✓
When you are done experimenting, enter `n` or `next` to go to the next exercise 🦀"
                    .bold()
                    .green(),
            ))?;
        }

        let line_width = size()?.0;
        let progress_bar = progress_bar(
            self.app_state.n_done(),
            self.app_state.exercises().len() as u16,
            line_width,
        )?;
        self.writer.write_fmt(format_args!(
            "{progress_bar}Current exercise: {}\n",
            self.app_state
                .current_exercise()
                .path
                .to_string_lossy()
                .bold(),
        ))?;

        self.show_prompt()?;

        Ok(())
    }

    pub fn show_hint(&mut self) -> Result<()> {
        self.show_hint = true;
        self.render()
    }

    pub fn handle_invalid_cmd(&mut self, cmd: &str) -> io::Result<()> {
        self.writer.write_all(b"Invalid command: ")?;
        self.writer.write_all(cmd.as_bytes())?;
        if cmd.len() > 1 {
            self.writer
                .write_all(b" (confusing input can occur after resizing the terminal)")?;
        }
        self.writer.write_all(b"\n")?;
        self.show_prompt()
    }
}
