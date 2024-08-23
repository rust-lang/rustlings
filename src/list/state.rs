use anyhow::{Context, Result};
use crossterm::{
    cursor::{MoveDown, MoveTo},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, BeginSynchronizedUpdate, EndSynchronizedUpdate},
    QueueableCommand,
};
use std::{
    fmt::Write as _,
    io::{self, StdoutLock, Write as _},
};

use crate::{app_state::AppState, term::clear_terminal, MAX_EXERCISE_NAME_LEN};

// +1 for padding.
const SPACE: &[u8] = &[b' '; MAX_EXERCISE_NAME_LEN + 1];

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
}

impl<'a> ListState<'a> {
    pub fn new(app_state: &'a mut AppState, stdout: &mut StdoutLock) -> io::Result<Self> {
        let name_col_width = app_state
            .exercises()
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .map_or(4, |max| max.max(4));

        clear_terminal(stdout)?;
        stdout.write_all(b"  Current  State    Name  ")?;
        stdout.write_all(&SPACE[..name_col_width - 4])?;
        stdout.write_all(b"Path\r\n")?;

        let selected = app_state.current_exercise_ind();
        let n_rows_with_filter = app_state.exercises().len();

        let mut slf = Self {
            message: String::with_capacity(128),
            filter: Filter::None,
            app_state,
            n_rows_with_filter,
            name_col_width,
            offset: selected.saturating_sub(10),
            selected: Some(selected),
        };

        slf.redraw(stdout)?;

        Ok(slf)
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
                .filter(|exercise| !exercise.done)
                .count(),
            Filter::Pending => self
                .app_state
                .exercises()
                .iter()
                .filter(|exercise| exercise.done)
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

    pub fn redraw(&mut self, stdout: &mut StdoutLock) -> io::Result<()> {
        stdout.queue(BeginSynchronizedUpdate)?;
        stdout.queue(MoveTo(0, 1))?;
        let (width, height) = terminal::size()?;
        let narrow = width < 95;
        let narrow_u16 = u16::from(narrow);
        let max_n_rows_to_display = height.saturating_sub(narrow_u16 + 4);

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
            .take(max_n_rows_to_display as usize);

        let mut n_displayed_rows: u16 = 0;
        let current_exercise_ind = self.app_state.current_exercise_ind();
        for (ind, exercise) in displayed_exercises {
            if self.selected == Some(n_displayed_rows as usize) {
                write!(stdout, "🦀")?;
            } else {
                stdout.write_all(b"  ")?;
            }

            if ind == current_exercise_ind {
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
            stdout.write_all(b"\r\n")?;

            n_displayed_rows += 1;
        }

        stdout.queue(MoveDown(max_n_rows_to_display - n_displayed_rows))?;

        // TODO
        // let message = if self.message.is_empty() {
        //     // Help footer.
        //     let mut text = Text::default();
        //     let mut spans = Vec::with_capacity(4);
        //     spans.push(Span::raw(
        //         "↓/j ↑/k home/g end/G │ <c>ontinue at │ <r>eset exercise │",
        //     ));

        //     if narrow {
        //         text.push_line(mem::take(&mut spans));
        //         spans.push(Span::raw("filter "));
        //     } else {
        //         spans.push(Span::raw(" filter "));
        //     }

        //     match self.filter {
        //         Filter::Done => {
        //             spans.push("<d>one".underlined().magenta());
        //             spans.push(Span::raw("/<p>ending"));
        //         }
        //         Filter::Pending => {
        //             spans.push(Span::raw("<d>one/"));
        //             spans.push("<p>ending".underlined().magenta());
        //         }
        //         Filter::None => spans.push(Span::raw("<d>one/<p>ending")),
        //     }

        //     spans.push(Span::raw(" │ <q>uit list"));
        //     text.push_line(spans);
        //     text
        // } else {
        //     Text::from(self.message.as_str().light_blue())
        // };

        stdout.queue(EndSynchronizedUpdate)?;
        stdout.flush()?;

        Ok(())
    }

    pub fn reset_selected(&mut self) -> Result<()> {
        let Some(selected) = self.selected else {
            return Ok(());
        };

        let ind = self
            .app_state
            .exercises()
            .iter()
            .enumerate()
            .filter_map(|(ind, exercise)| match self.filter {
                Filter::Done => exercise.done.then_some(ind),
                Filter::Pending => (!exercise.done).then_some(ind),
                Filter::None => Some(ind),
            })
            .nth(selected)
            .context("Invalid selection index")?;

        let exercise_path = self.app_state.reset_exercise_by_ind(ind)?;
        write!(self.message, "The exercise {exercise_path} has been reset")?;

        Ok(())
    }

    // Return `true` if there was something to select.
    pub fn selected_to_current_exercise(&mut self) -> Result<bool> {
        let Some(selected) = self.selected else {
            self.message.push_str("Nothing selected to continue at!");
            return Ok(false);
        };

        let (ind, _) = self
            .app_state
            .exercises()
            .iter()
            .enumerate()
            .filter(|(_, exercise)| match self.filter {
                Filter::Done => exercise.done,
                Filter::Pending => !exercise.done,
                Filter::None => true,
            })
            .nth(selected)
            .context("Invalid selection index")?;

        self.app_state.set_current_exercise_ind(ind)?;
        Ok(true)
    }
}
