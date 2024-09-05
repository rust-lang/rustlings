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
    term::{progress_bar, CountedWrite, MaxLenWriter},
};

use super::scroll_state::ScrollState;

const COL_SPACING: usize = 2;

fn next_ln(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout
        .queue(Clear(ClearType::UntilNewLine))?
        .queue(MoveToNextLine(1))?;
    Ok(())
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
    pub search_query: String,
    app_state: &'a mut AppState,
    scroll_state: ScrollState,
    name_col_padding: Vec<u8>,
    filter: Filter,
    term_width: u16,
    term_height: u16,
    show_footer: bool,
}

impl<'a> ListState<'a> {
    pub fn build(app_state: &'a mut AppState, stdout: &mut StdoutLock) -> Result<Self> {
        stdout.queue(Clear(ClearType::All))?;

        let name_col_title_len = 4;
        let name_col_width = app_state
            .exercises()
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .map_or(name_col_title_len, |max| max.max(name_col_title_len));
        let name_col_padding = vec![b' '; name_col_width + COL_SPACING];

        let filter = Filter::None;
        let n_rows_with_filter = app_state.exercises().len();
        let selected = app_state.current_exercise_ind();

        let (width, height) = terminal::size().context("Failed to get the terminal size")?;
        let scroll_state = ScrollState::new(n_rows_with_filter, Some(selected), 5);

        let mut slf = Self {
            message: String::with_capacity(128),
            search_query: String::new(),
            app_state,
            scroll_state,
            name_col_padding,
            filter,
            // Set by `set_term_size`
            term_width: 0,
            term_height: 0,
            show_footer: true,
        };

        slf.set_term_size(width, height);
        slf.draw(stdout)?;

        Ok(slf)
    }

    pub fn set_term_size(&mut self, width: u16, height: u16) {
        self.term_width = width;
        self.term_height = height;

        if height == 0 {
            return;
        }

        let header_height = 1;
        // 1 progress bar, 2 footer message lines.
        let footer_height = 3;
        self.show_footer = height > header_height + footer_height;

        self.scroll_state.set_max_n_rows_to_display(
            height.saturating_sub(header_height + u16::from(self.show_footer) * footer_height)
                as usize,
        );
    }

    fn draw_rows(
        &self,
        stdout: &mut StdoutLock,
        filtered_exercises: impl Iterator<Item = (usize, &'a Exercise)>,
    ) -> io::Result<usize> {
        let current_exercise_ind = self.app_state.current_exercise_ind();
        let row_offset = self.scroll_state.offset();
        let mut n_displayed_rows = 0;

        for (exercise_ind, exercise) in filtered_exercises
            .skip(row_offset)
            .take(self.scroll_state.max_n_rows_to_display())
        {
            let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);

            if self.scroll_state.selected() == Some(row_offset + n_displayed_rows) {
                writer.stdout.queue(SetBackgroundColor(Color::Rgb {
                    r: 40,
                    g: 40,
                    b: 40,
                }))?;
                // The crab emoji has the width of two ascii chars.
                writer.add_to_len(2);
                writer.stdout.write_all("🦀".as_bytes())?;
            } else {
                writer.write_ascii(b"  ")?;
            }

            if exercise_ind == current_exercise_ind {
                writer.stdout.queue(SetForegroundColor(Color::Red))?;
                writer.write_ascii(b">>>>>>>  ")?;
            } else {
                writer.write_ascii(b"         ")?;
            }

            if exercise.done {
                writer.stdout.queue(SetForegroundColor(Color::Green))?;
                writer.write_ascii(b"DONE     ")?;
            } else {
                writer.stdout.queue(SetForegroundColor(Color::Yellow))?;
                writer.write_ascii(b"PENDING  ")?;
            }

            writer.stdout.queue(SetForegroundColor(Color::Reset))?;

            writer.write_str(exercise.name)?;
            writer.write_ascii(&self.name_col_padding[exercise.name.len()..])?;

            // The list links aren't shown correctly in VS Code on Windows.
            // But VS Code shows its own links anyway.
            if self.app_state.vs_code() {
                writer.write_str(exercise.path)?;
            } else {
                exercise.terminal_file_link(&mut writer)?;
            }

            next_ln(stdout)?;
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
        let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);
        writer.write_ascii(b"  Current  State    Name")?;
        writer.write_ascii(&self.name_col_padding[4..])?;
        writer.write_ascii(b"Path")?;
        next_ln(stdout)?;

        // Rows
        let iter = self.app_state.exercises().iter().enumerate();
        let n_displayed_rows = match self.filter {
            Filter::Done => self.draw_rows(stdout, iter.filter(|(_, exercise)| exercise.done))?,
            Filter::Pending => {
                self.draw_rows(stdout, iter.filter(|(_, exercise)| !exercise.done))?
            }
            Filter::None => self.draw_rows(stdout, iter)?,
        };

        for _ in 0..self.scroll_state.max_n_rows_to_display() - n_displayed_rows {
            next_ln(stdout)?;
        }

        if self.show_footer {
            progress_bar(
                &mut MaxLenWriter::new(stdout, self.term_width as usize),
                self.app_state.n_done(),
                self.app_state.exercises().len() as u16,
                self.term_width,
            )?;
            next_ln(stdout)?;

            let mut writer = MaxLenWriter::new(stdout, self.term_width as usize);
            if self.message.is_empty() {
                // Help footer message
                if self.scroll_state.selected().is_some() {
                    writer.write_str("↓/j ↑/k home/g end/G | <c>ontinue at | <r>eset exercise")?;
                    next_ln(stdout)?;
                    writer = MaxLenWriter::new(stdout, self.term_width as usize);

                    writer.write_ascii(b"<s>earch | filter ")?;
                } else {
                    // Nothing selected (and nothing shown), so only display filter and quit.
                    writer.write_ascii(b"filter ")?;
                }

                match self.filter {
                    Filter::Done => {
                        writer
                            .stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        writer.write_ascii(b"<d>one")?;
                        writer.stdout.queue(ResetColor)?;
                        writer.write_ascii(b"/<p>ending")?;
                    }
                    Filter::Pending => {
                        writer.write_ascii(b"<d>one/")?;
                        writer
                            .stdout
                            .queue(SetForegroundColor(Color::Magenta))?
                            .queue(SetAttribute(Attribute::Underlined))?;
                        writer.write_ascii(b"<p>ending")?;
                        writer.stdout.queue(ResetColor)?;
                    }
                    Filter::None => writer.write_ascii(b"<d>one/<p>ending")?,
                }

                writer.write_ascii(b" | <q>uit list")?;
            } else {
                writer.stdout.queue(SetForegroundColor(Color::Magenta))?;
                writer.write_str(&self.message)?;
                stdout.queue(ResetColor)?;
                next_ln(stdout)?;
            }

            next_ln(stdout)?;
        }

        stdout.queue(EndSynchronizedUpdate)?.flush()
    }

    fn update_rows(&mut self) {
        let n_rows = match self.filter {
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

        self.scroll_state.set_n_rows(n_rows);
    }

    #[inline]
    pub fn filter(&self) -> Filter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
        self.update_rows();
    }

    #[inline]
    pub fn select_next(&mut self) {
        self.scroll_state.select_next();
    }

    #[inline]
    pub fn select_previous(&mut self) {
        self.scroll_state.select_previous();
    }

    #[inline]
    pub fn select_first(&mut self) {
        self.scroll_state.select_first();
    }

    #[inline]
    pub fn select_last(&mut self) {
        self.scroll_state.select_last();
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
        let Some(selected) = self.scroll_state.selected() else {
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

    pub fn apply_search_query(&mut self) {
        self.message.push_str("search:");
        self.message.push_str(&self.search_query);
        self.message.push('|');

        if self.search_query.is_empty() {
            return;
        }

        let is_search_result = |exercise: &Exercise| exercise.name.contains(&self.search_query);
        let mut iter = self.app_state.exercises().iter();
        let ind = match self.filter {
            Filter::None => iter.position(is_search_result),
            Filter::Done => iter
                .filter(|exercise| exercise.done)
                .position(is_search_result),
            Filter::Pending => iter
                .filter(|exercise| !exercise.done)
                .position(is_search_result),
        };

        match ind {
            Some(exercise_ind) => self.scroll_state.set_selected(exercise_ind),
            None => self.message.push_str(" (not found)"),
        }
    }

    // Return `true` if there was something to select.
    pub fn selected_to_current_exercise(&mut self) -> Result<bool> {
        let Some(selected) = self.scroll_state.selected() else {
            self.message.push_str("Nothing selected to continue at!");
            return Ok(false);
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        self.app_state.set_current_exercise_ind(exercise_ind)?;

        Ok(true)
    }
}
