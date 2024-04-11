use anyhow::Result;
use crossterm::{
    style::Stylize,
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, StdoutLock, Write};

use crate::{app_state::AppState, progress_bar::progress_bar};

pub struct WatchState<'a> {
    writer: StdoutLock<'a>,
    app_state: &'a mut AppState,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    message: Option<String>,
    hint_displayed: bool,
}

impl<'a> WatchState<'a> {
    pub fn new(app_state: &'a mut AppState) -> Self {
        let writer = io::stdout().lock();

        Self {
            writer,
            app_state,
            stdout: None,
            stderr: None,
            message: None,
            hint_displayed: false,
        }
    }

    #[inline]
    pub fn into_writer(self) -> StdoutLock<'a> {
        self.writer
    }

    pub fn run_current_exercise(&mut self) -> Result<bool> {
        let output = self.app_state.current_exercise().run()?;
        self.stdout = Some(output.stdout);

        if !output.status.success() {
            self.stderr = Some(output.stderr);
            return Ok(false);
        }

        self.stderr = None;

        Ok(true)
    }

    pub fn run_exercise_with_ind(&mut self, exercise_ind: usize) -> Result<bool> {
        self.app_state.set_current_exercise_ind(exercise_ind)?;
        self.run_current_exercise()
    }

    fn show_prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(b"\n\n")?;

        if !self.hint_displayed {
            self.writer.write_fmt(format_args!("{}int/", 'h'.bold()))?;
        }

        self.writer
            .write_fmt(format_args!("{}ist/{}uit? ", 'l'.bold(), 'q'.bold()))?;

        self.writer.flush()
    }

    pub fn render(&mut self) -> Result<()> {
        // Prevent having the first line shifted after clearing because of the prompt.
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

        if let Some(message) = &self.message {
            self.writer.write_all(message.as_bytes())?;
        }

        self.writer.write_all(b"\n")?;

        if self.hint_displayed {
            self.writer
                .write_fmt(format_args!("\n{}\n", "Hint".bold().cyan().underlined()))?;
            self.writer
                .write_all(self.app_state.current_exercise().hint.as_bytes())?;
            self.writer.write_all(b"\n\n")?;
        }

        let line_width = size()?.0;
        let progress_bar = progress_bar(
            self.app_state.n_done(),
            self.app_state.exercises().len() as u16,
            line_width,
        )?;
        self.writer.write_all(progress_bar.as_bytes())?;

        self.writer.write_all(b"Current exercise: ")?;
        self.writer.write_fmt(format_args!(
            "{}",
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
        self.hint_displayed = true;
        self.render()
    }

    pub fn handle_invalid_cmd(&mut self, cmd: &str) -> io::Result<()> {
        self.writer.write_all(b"Invalid command: ")?;
        self.writer.write_all(cmd.as_bytes())?;
        if cmd.len() > 1 {
            self.writer
                .write_all(b" (confusing input can occur after resizing the terminal)")?;
        }
        self.show_prompt()
    }
}
