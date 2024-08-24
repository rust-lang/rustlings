use anyhow::{Context, Result};
use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate},
    QueueableCommand,
};
use std::{
    fmt::Write as _,
    io::{self, StdoutLock, Write},
};

use crate::{app_state::AppState, term::progress_bar, MAX_EXERCISE_NAME_LEN};

fn next_ln<const CLEAR_LAST_CHAR: bool>(stdout: &mut StdoutLock) -> io::Result<()> {
    if CLEAR_LAST_CHAR {
        // Avoids having the last written char as the last displayed one when
        // the written width is higher than the terminal width.
        // Happens on the Gnome terminal for example.
        stdout.write_all(b" ")?;
    }

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
    pub message: String,
    filter: Filter,
    app_state: &'a mut AppState,
    n_rows_with_filter: usize,
    name_col_width: usize,
    offset: usize,
    selected: Option<usize>,
    term_width: u16,
    term_height: u16,
    separator: Vec<u8>,
}

impl<'a> ListState<'a> {
    pub fn new(app_state: &'a mut AppState, stdout: &mut StdoutLock) -> io::Result<Self> {
        let (term_width, term_height) = terminal::size()?;
        stdout.queue(Clear(ClearType::All))?;

        let name_col_width = app_state
            .exercises()
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .map_or(4, |max| max.max(4));

        let n_rows_with_filter = app_state.exercises().len();
        let selected = app_state.current_exercise_ind();

        let mut slf = Self {
            message: String::with_capacity(128),
            filter: Filter::None,
            app_state,
            n_rows_with_filter,
            name_col_width,
            offset: selected.saturating_sub(10),
            selected: Some(selected),
            term_width,
            term_height,
            separator: "─".as_bytes().repeat(term_width as usize),
        };

        slf.redraw(stdout)?;

        Ok(slf)
    }

    pub fn redraw(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        if self.term_height == 0 {
            return Ok(());
        }

        stdout.queue(BeginSynchronizedUpdate)?.queue(MoveTo(0, 0))?;

        // +1 for padding.
        const SPACE: &[u8] = &[b' '; MAX_EXERCISE_NAME_LEN + 1];
        stdout.write_all(b"  Current  State    Name")?;
        stdout.write_all(&SPACE[..self.name_col_width - 2])?;
        stdout.write_all(b"Path")?;
        next_ln::<true>(stdout)?;

        let narrow = self.term_width < 96;
        let show_footer = self.term_height > 6;
        let max_n_rows_to_display =
            (self.term_height - 1 - u16::from(show_footer) * (4 + u16::from(narrow))) as usize;

        let displayed_exercises = self
            .app_state
            .exercises()
            .iter()
            .enumerate()
            .filter(|(_, exercise)| match self.filter {
                Filter::Done => exercise.done,
                Filter::Pending => !exercise.done,
                Filter::None => true,
            })
            .skip(self.offset)
            .take(max_n_rows_to_display);

        let current_exercise_ind = self.app_state.current_exercise_ind();
        let mut n_displayed_rows = 0;
        for (exercise_ind, exercise) in displayed_exercises {
            if self.selected == Some(n_displayed_rows) {
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
                stdout.queue(SetForegroundColor(Color::Yellow))?;
                stdout.write_all(b"DONE     ")?;
            } else {
                stdout.queue(SetForegroundColor(Color::Green))?;
                stdout.write_all(b"PENDING  ")?;
            }

            stdout.queue(ResetColor)?;

            stdout.write_all(exercise.name.as_bytes())?;
            stdout.write_all(&SPACE[..self.name_col_width + 2 - exercise.name.len()])?;

            stdout.write_all(exercise.path.as_bytes())?;

            next_ln::<true>(stdout)?;
            n_displayed_rows += 1;
        }

        for _ in 0..max_n_rows_to_display - n_displayed_rows {
            next_ln::<false>(stdout)?;
        }

        if show_footer {
            stdout.write_all(&self.separator)?;
            next_ln::<false>(stdout)?;

            progress_bar(
                stdout,
                self.app_state.n_done(),
                self.app_state.exercises().len() as u16,
                self.term_width,
            )?;
            next_ln::<false>(stdout)?;

            stdout.write_all(&self.separator)?;
            next_ln::<false>(stdout)?;

            if self.message.is_empty() {
                // Help footer.
                stdout.write_all(
                    "↓/j ↑/k home/g end/G │ <c>ontinue at │ <r>eset exercise │".as_bytes(),
                )?;
                if narrow {
                    next_ln::<true>(stdout)?;
                    stdout.write_all(b"filter ")?;
                } else {
                    stdout.write_all(b" filter ")?;
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
                stdout.write_all(" │ <q>uit list".as_bytes())?;
                next_ln::<true>(stdout)?;
            } else {
                stdout.queue(SetForegroundColor(Color::Magenta))?;
                stdout.write_all(self.message.as_bytes())?;
                stdout.queue(ResetColor)?;
                next_ln::<true>(stdout)?;
                if narrow {
                    next_ln::<false>(stdout)?;
                }
            }
        }

        stdout.queue(EndSynchronizedUpdate)?.flush()
    }

    pub fn set_term_size(&mut self, width: u16, height: u16) {
        self.term_width = width;
        self.term_height = height;
        self.separator = "─".as_bytes().repeat(width as usize);
    }

    #[inline]
    pub fn filter(&self) -> Filter {
        self.filter
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
        self.n_rows_with_filter = match filter {
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
            self.selected = None;
        } else {
            self.selected = Some(
                self.selected
                    .map_or(0, |selected| selected.min(self.n_rows_with_filter - 1)),
            );
        }
    }

    pub fn select_next(&mut self) {
        if self.n_rows_with_filter > 0 {
            let next = self.selected.map_or(0, |selected| {
                (selected + 1).min(self.n_rows_with_filter - 1)
            });
            self.selected = Some(next);
        }
    }

    pub fn select_previous(&mut self) {
        if self.n_rows_with_filter > 0 {
            let previous = self
                .selected
                .map_or(0, |selected| selected.saturating_sub(1));
            self.selected = Some(previous);
        }
    }

    pub fn select_first(&mut self) {
        if self.n_rows_with_filter > 0 {
            self.selected = Some(0);
        }
    }

    pub fn select_last(&mut self) {
        if self.n_rows_with_filter > 0 {
            self.selected = Some(self.n_rows_with_filter - 1);
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
        let Some(selected) = self.selected else {
            self.message.push_str("Nothing selected to reset!");
            return Ok(());
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        let exercise_path = self.app_state.reset_exercise_by_ind(exercise_ind)?;
        write!(self.message, "The exercise {exercise_path} has been reset")?;

        Ok(())
    }

    // Return `true` if there was something to select.
    pub fn selected_to_current_exercise(&mut self) -> Result<bool> {
        let Some(selected) = self.selected else {
            self.message.push_str("Nothing selected to continue at!");
            return Ok(false);
        };

        let exercise_ind = self.selected_to_exercise_ind(selected)?;
        self.app_state.set_current_exercise_ind(exercise_ind)?;
        Ok(true)
    }
}
