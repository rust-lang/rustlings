use anyhow::{Context, Result};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    style::{Attribute, Color, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
    QueueableCommand,
};
use std::{
    fmt::Write as _,
    io::{self, StdoutLock, Write},
};

use crate::{
    app_state::AppState,
    exercise::Exercise,
    term::{progress_bar, terminal_file_link},
    MAX_EXERCISE_NAME_LEN,
};

const MAX_SCROLL_PADDING: usize = 5;
// +1 for column padding.
const SPACE: &[u8] = &[b' '; MAX_EXERCISE_NAME_LEN + 1];

fn next_ln(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(MoveToNextLine(1))?;
    Ok(())
}

// Avoids having the last written char as the last displayed one when the
// written width is higher than the terminal width.
// Happens on the Gnome terminal for example.
fn next_ln_overwrite(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.write_all(b" ")?;
    next_ln(stdout)
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    Done,
    Pending,
    None,
}

pub struct ListState<'a> {
    /// Footer message to be displayed if not empty.
    pub message: String,
    app_state: &'a mut AppState,
    name_col_width: usize,
    filter: Filter,
    n_rows_with_filter: usize,
    /// Selected row out of the filtered ones.
    selected_row: Option<usize>,
    row_offset: usize,
    term_width: u16,
    term_height: u16,
    separator_line: Vec<u8>,
    narrow_term: bool,
    show_footer: bool,
    max_n_rows_to_display: usize,
    scroll_padding: usize,
}

impl<'a> ListState<'a> {
    pub fn new(app_state: &'a mut AppState, stdout: &mut StdoutLock) -> io::Result<Self> {
        stdout.queue(Clear(ClearType::All))?;

        let name_col_title_len = 4;
        let name_col_width = app_state
            .exercises()
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .map_or(name_col_title_len, |max| max.max(name_col_title_len));

        let filter = Filter::None;
        let n_rows_with_filter = app_state.exercises().len();
        let selected = app_state.current_exercise_ind();

        let mut slf = Self {
            message: String::with_capacity(128),
            app_state,
            name_col_width,
            filter,
            n_rows_with_filter,
            selected_row: Some(selected),
            row_offset: selected.saturating_sub(MAX_SCROLL_PADDING),
            // Set by `set_term_size`
            term_width: 0,
            term_height: 0,
            separator_line: Vec::new(),
            narrow_term: false,
            show_footer: true,
            max_n_rows_to_display: 0,
            scroll_padding: 0,
        };

        let (width, height) = terminal::size()?;
        slf.set_term_size(width, height);
        slf.draw(stdout)?;

        Ok(slf)
    }

    fn update_offset(&mut self) {
        let Some(selected) = self.selected_row else {
            return;
        };

        let min_offset = (selected + self.scroll_padding)
            .saturating_sub(self.max_n_rows_to_display.saturating_sub(1));
        let max_offset = selected.saturating_sub(self.scroll_padding);
        let global_max_offset = self
            .n_rows_with_filter
            .saturating_sub(self.max_n_rows_to_display);

        self.row_offset = self
            .row_offset
            .max(min_offset)
            .min(max_offset)
            .min(global_max_offset);
    }

    pub fn set_term_size(&mut self, width: u16, height: u16) {
        self.term_width = width;
        self.term_height = height;

        if height == 0 {
            return;
        }

        let wide_help_footer_width = 95;
        // The help footer is shorter when nothing is selected.
        self.narrow_term = width < wide_help_footer_width && self.selected_row.is_some();

        let header_height = 1;
        // 2 separator, 1 progress bar, 1-2 footer message.
        let footer_height = 4 + u16::from(self.narrow_term);
        self.show_footer = height > header_height + footer_height;

        if self.show_footer {
            self.separator_line = "─".as_bytes().repeat(width as usize);
        }

        self.max_n_rows_to_display = height
            .saturating_sub(header_height + u16::from(self.show_footer) * footer_height)
            as usize;

        self.scroll_padding = (self.max_n_rows_to_display / 4).min(MAX_SCROLL_PADDING);

        self.update_offset();
    }

    fn draw_rows(
        &self,
        stdout: &mut StdoutLock,
        filtered_exercises: impl Iterator<Item = (usize, &'a Exercise)>,
    ) -> io::Result<usize> {
        let current_exercise_ind = self.app_state.current_exercise_ind();
        let mut n_displayed_rows = 0;

        for (exercise_ind, exercise) in filtered_exercises
            .skip(self.row_offset)
            .take(self.max_n_rows_to_display)
        {
            if self.selected_row == Some(self.row_offset + n_displayed_rows) {
                stdout.queue(SetBackgroundColor(Color::Rgb {
                    r: 50,
                    g: 50,
                    b: 50,
                }))?;
                stdout.write_all("🦀".as_bytes())?;
            } else {
                stdout.write_all(b"  ")?;
            }

            if exercise_ind == current_exercise_ind {
                stdout.queue(SetForegroundColor(Color::Red))?;
                stdout.write_all(b">>>>>>>  ")?;
            } else {
                stdout.write_all(b"         ")?;
            }

            if exercise.done {
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.write_all(b"DONE     ")?;
            } else {
                stdout.queue(SetForegroundColor(Color::Yellow))?;
                stdout.write_all(b"PENDING  ")?;
            }

            stdout.queue(SetForegroundColor(Color::Reset))?;

            stdout.write_all(exercise.name.as_bytes())?;
            stdout.write_all(&SPACE[..self.name_col_width + 2 - exercise.name.len()])?;

            terminal_file_link(stdout, exercise.path, Color::Blue)?;

            next_ln_overwrite(stdout)?;
            stdout.queue(ResetColor)?;
            n_displayed_rows += 1;
        }

        Ok(n_displayed_rows)
    }

    pub fn draw(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.term_height == 0 {
            return Ok(());
        }

        stdout.queue(BeginSynchronizedUpdate)?.queue(MoveTo(0, 0))?;

        // Header
        stdout.write_all(b"  Current  State    Name")?;
        stdout.write_all(&SPACE[..self.name_col_width - 2])?;
        stdout.write_all(b"Path")?;
        next_ln_overwrite(stdout)?;

        // Rows
        let iter = self.app_state.exercises().iter().enumerate();
        let n_displayed_rows = match self.filter {
            Filter::Done => self.draw_rows(stdout, iter.filter(|(_, exercise)| exercise.done))?,
            Filter::Pending => {
                self.draw_rows(stdout, iter.filter(|(_, exercise)| !exercise.done))?
            }
            Filter::None => self.draw_rows(stdout, iter)?,
        };

        for _ in 0..self.max_n_rows_to_display - n_displayed_rows {
            next_ln(stdout)?;
        }

        if self.show_footer {
            stdout.write_all(&self.separator_line)?;
            next_ln(stdout)?;

            progress_bar(
                stdout,
                self.app_state.n_done(),
                self.app_state.exercises().len() as u16,
                self.term_width,
            )?;
            next_ln(stdout)?;

            stdout.write_all(&self.separator_line)?;
            next_ln(stdout)?;

            if self.message.is_empty() {
                // Help footer message
                if self.selected_row.is_some() {
                    stdout.write_all(
                        "↓/j ↑/k home/g end/G | <c>ontinue at | <r>eset exercise".as_bytes(),
                    )?;
                    if self.narrow_term {
                        next_ln_overwrite(stdout)?;
                        stdout.write_all(b"filter ")?;
                    } else {
                        stdout.write_all(b" | filter ")?;
                    }
                } else {
                    // Nothing selected (and nothing shown), so only display filter and quit.
                    stdout.write_all(b"filter ")?;
                }

                match self.filter {
                    Filter::Done => {
                        stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        stdout.write_all(b"<d>one")?;
                        stdout.queue(ResetColor)?;
                        stdout.write_all(b"/<p>ending")?;
                    }
                    Filter::Pending => {
                        stdout.write_all(b"<d>one/")?;
                        stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        stdout.write_all(b"<p>ending")?;
                        stdout.queue(ResetColor)?;
                    }
                    Filter::None => stdout.write_all(b"<d>one/<p>ending")?,
                }

                stdout.write_all(b" | <q>uit list")?;

                if self.narrow_term {
                    next_ln_overwrite(stdout)?;
                } else {
                    next_ln(stdout)?;
                }
            } else {
                stdout.queue(SetForegroundColor(Color::Magenta))?;
                stdout.write_all(self.message.as_bytes())?;
                stdout.queue(ResetColor)?;
                next_ln_overwrite(stdout)?;
                if self.narrow_term {
                    next_ln(stdout)?;
                }
            }
        }

        stdout.queue(EndSynchronizedUpdate)?.flush()
    }

    fn set_selected(&mut self, selected: usize) {
        self.selected_row = Some(selected);
        self.update_offset();
    }

    fn update_rows(&mut self) {
        self.n_rows_with_filter = match self.filter {
            Filter::Done => self
                .app_state
                .exercises()
                .iter()
                .filter(|exercise| exercise.done)
                .count(),
            Filter::Pending => self
                .app_state
                .exercises()
                .iter()
                .filter(|exercise| !exercise.done)
                .count(),
            Filter::None => self.app_state.exercises().len(),
        };

        if self.n_rows_with_filter == 0 {
            self.selected_row = None;
            return;
        }

        self.set_selected(
            self.selected_row
                .map_or(0, |selected| selected.min(self.n_rows_with_filter - 1)),
        );
    }

    #[inline]
    pub fn filter(&self) -> Filter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
        self.update_rows();
    }

    pub fn select_next(&mut self) {
        if let Some(selected) = self.selected_row {
            self.set_selected((selected + 1).min(self.n_rows_with_filter - 1));
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(selected) = self.selected_row {
            self.set_selected(selected.saturating_sub(1));
        }
    }

    pub fn select_first(&mut self) {
        if self.n_rows_with_filter > 0 {
            self.set_selected(0);
        }
    }

    pub fn select_last(&mut self) {
        if self.n_rows_with_filter > 0 {
            self.set_selected(self.n_rows_with_filter - 1);
        }
    }

    fn selected_to_exercise_ind(&self, selected: usize) -> Result<usize> {
        match self.filter {
            Filter::Done => self
                .app_state
                .exercises()
                .iter()
                .enumerate()
                .filter(|(_, exercise)| exercise.done)
                .nth(selected)
                .context("Invalid selection index")
                .map(|(ind, _)| ind),
            Filter::Pending => self
                .app_state
                .exercises()
                .iter()
                .enumerate()
                .filter(|(_, exercise)| !exercise.done)
                .nth(selected)
                .context("Invalid selection index")
                .map(|(ind, _)| ind),
            Filter::None => Ok(selected),
        }
    }

    pub fn reset_selected(&mut self) -> Result<()> {
        let Some(selected) = self.selected_row else {
            self.message.push_str("Nothing selected to reset!");
            return Ok(());
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        let exercise_name = self.app_state.reset_exercise_by_ind(exercise_ind)?;
        self.update_rows();
        write!(
            self.message,
            "The exercise `{exercise_name}` has been reset",
        )?;

        Ok(())
    }

    // Return `true` if there was something to select.
    pub fn selected_to_current_exercise(&mut self) -> Result<bool> {
        let Some(selected) = self.selected_row else {
            self.message.push_str("Nothing selected to continue at!");
            return Ok(false);
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        self.app_state.set_current_exercise_ind(exercise_ind)?;

        Ok(true)
    }
}
