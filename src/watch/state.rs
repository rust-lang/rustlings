use anyhow::Result;
use crossterm::{
    style::{style, Stylize},
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{self, StdoutLock, Write};

use crate::{
    app_state::{AppState, ExercisesProgress},
    exercise::OUTPUT_CAPACITY,
    progress_bar::progress_bar,
    terminal_link::TerminalFileLink,
};

enum DoneStatus {
    DoneWithSolution(String),
    DoneWithoutSolution,
    Pending,
}

pub struct WatchState<'a> {
    writer: StdoutLock<'a>,
    app_state: &'a mut AppState,
    output: Vec<u8>,
    show_hint: bool,
    done_status: DoneStatus,
    manual_run: bool,
}

impl<'a> WatchState<'a> {
    pub fn new(app_state: &'a mut AppState, manual_run: bool) -> Self {
        let writer = io::stdout().lock();

        Self {
            writer,
            app_state,
            output: Vec::with_capacity(OUTPUT_CAPACITY),
            show_hint: false,
            done_status: DoneStatus::Pending,
            manual_run,
        }
    }

    #[inline]
    pub fn into_writer(self) -> StdoutLock<'a> {
        self.writer
    }

    pub fn run_current_exercise(&mut self) -> Result<()> {
        self.show_hint = false;

        let success = self.app_state.current_exercise().run(&mut self.output)?;
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

        self.render()
    }

    pub fn run_exercise_with_ind(&mut self, exercise_ind: usize) -> Result<()> {
        self.app_state.set_current_exercise_ind(exercise_ind)?;
        self.run_current_exercise()
    }

    pub fn next_exercise(&mut self) -> Result<ExercisesProgress> {
        if matches!(self.done_status, DoneStatus::Pending) {
            self.writer
                .write_all(b"The current exercise isn't done yet\n")?;
            self.show_prompt()?;
            return Ok(ExercisesProgress::Pending);
        }

        self.app_state.done_current_exercise(&mut self.writer)
    }

    fn show_prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(b"\n")?;

        if self.manual_run {
            self.writer.write_fmt(format_args!("{}un/", 'r'.bold()))?;
        }

        if !matches!(self.done_status, DoneStatus::Pending) {
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

        self.writer.write_all(&self.output)?;
        self.writer.write_all(b"\n")?;

        if self.show_hint {
            self.writer.write_fmt(format_args!(
                "{}\n{}\n\n",
                "Hint".bold().cyan().underlined(),
                self.app_state.current_exercise().hint,
            ))?;
        }

        if !matches!(self.done_status, DoneStatus::Pending) {
            self.writer.write_fmt(format_args!(
                "{}\n\n",
                "Exercise done ✓
When you are done experimenting, enter `n` or `next` to go to the next exercise 🦀"
                    .bold()
                    .green(),
            ))?;
        }

        if let DoneStatus::DoneWithSolution(solution_path) = &self.done_status {
            self.writer.write_fmt(format_args!(
                "A solution file can be found at {}\n\n",
                style(TerminalFileLink(solution_path)).underlined().green()
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
            self.app_state.current_exercise().terminal_link(),
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
