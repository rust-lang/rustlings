use anyhow::{Context, Result};
use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, Paragraph, Row, Table, TableState},
    Frame,
};
use std::fmt::Write;

use crate::{app_state::AppState, progress_bar::progress_bar_ratatui};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    Done,
    Pending,
    None,
}

pub struct UiState<'a> {
    pub table: Table<'static>,
    pub message: String,
    pub filter: Filter,
    app_state: &'a mut AppState,
    table_state: TableState,
    n_rows: usize,
}

impl<'a> UiState<'a> {
    pub fn with_updated_rows(mut self) -> Self {
        let current_exercise_ind = self.app_state.current_exercise_ind();

        self.n_rows = 0;
        let rows = self
            .app_state
            .exercises()
            .iter()
            .enumerate()
            .filter_map(|(ind, exercise)| {
                let exercise_state = if exercise.done {
                    if self.filter == Filter::Pending {
                        return None;
                    }

                    "DONE".green()
                } else {
                    if self.filter == Filter::Done {
                        return None;
                    }

                    "PENDING".yellow()
                };

                self.n_rows += 1;

                let next = if ind == current_exercise_ind {
                    ">>>>".bold().red()
                } else {
                    Span::default()
                };

                Some(Row::new([
                    next,
                    exercise_state,
                    Span::raw(exercise.name),
                    Span::raw(exercise.path),
                ]))
            });

        self.table = self.table.rows(rows);

        if self.n_rows == 0 {
            self.table_state.select(None);
        } else {
            self.table_state.select(Some(
                self.table_state
                    .selected()
                    .map_or(0, |selected| selected.min(self.n_rows - 1)),
            ));
        }

        self
    }

    pub fn new(app_state: &'a mut AppState) -> Self {
        let header = Row::new(["Next", "State", "Name", "Path"]);

        let max_name_len = app_state
            .exercises()
            .iter()
            .map(|exercise| exercise.name.len())
            .max()
            .unwrap_or(4) as u16;

        let widths = [
            Constraint::Length(4),
            Constraint::Length(7),
            Constraint::Length(max_name_len),
            Constraint::Fill(1),
        ];

        let table = Table::default()
            .widths(widths)
            .header(header)
            .column_spacing(2)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::new().bg(ratatui::style::Color::Rgb(50, 50, 50)))
            .highlight_symbol("🦀")
            .block(Block::default().borders(Borders::BOTTOM));

        let selected = app_state.current_exercise_ind();
        let table_state = TableState::default()
            .with_offset(selected.saturating_sub(10))
            .with_selected(Some(selected));

        let filter = Filter::None;
        let n_rows = app_state.exercises().len();

        let slf = Self {
            table,
            message: String::with_capacity(128),
            filter,
            app_state,
            table_state,
            n_rows,
        };

        slf.with_updated_rows()
    }

    pub fn select_next(&mut self) {
        if self.n_rows > 0 {
            let next = self
                .table_state
                .selected()
                .map_or(0, |selected| (selected + 1).min(self.n_rows - 1));
            self.table_state.select(Some(next));
        }
    }

    pub fn select_previous(&mut self) {
        if self.n_rows > 0 {
            let previous = self
                .table_state
                .selected()
                .map_or(0, |selected| selected.saturating_sub(1));
            self.table_state.select(Some(previous));
        }
    }

    pub fn select_first(&mut self) {
        if self.n_rows > 0 {
            self.table_state.select(Some(0));
        }
    }

    pub fn select_last(&mut self) {
        if self.n_rows > 0 {
            self.table_state.select(Some(self.n_rows - 1));
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        let area = frame.size();

        frame.render_stateful_widget(
            &self.table,
            Rect {
                x: 0,
                y: 0,
                width: area.width,
                height: area.height - 3,
            },
            &mut self.table_state,
        );

        frame.render_widget(
            Paragraph::new(progress_bar_ratatui(
                self.app_state.n_done(),
                self.app_state.exercises().len() as u16,
                area.width,
            )?)
            .block(Block::default().borders(Borders::BOTTOM)),
            Rect {
                x: 0,
                y: area.height - 3,
                width: area.width,
                height: 2,
            },
        );

        let message = if self.message.is_empty() {
            // Help footer.
            let mut spans = Vec::with_capacity(4);
            spans.push(Span::raw(
                "↓/j ↑/k home/g end/G │ <c>ontinue at │ <r>eset │ filter ",
            ));
            match self.filter {
                Filter::Done => {
                    spans.push("<d>one".underlined().magenta());
                    spans.push(Span::raw("/<p>ending"));
                }
                Filter::Pending => {
                    spans.push(Span::raw("<d>one/"));
                    spans.push("<p>ending".underlined().magenta());
                }
                Filter::None => spans.push(Span::raw("<d>one/<p>ending")),
            }
            spans.push(Span::raw(" │ <q>uit"));
            Line::from(spans)
        } else {
            Line::from(self.message.as_str().light_blue())
        };
        frame.render_widget(
            message,
            Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            },
        );

        Ok(())
    }

    pub fn with_reset_selected(mut self) -> Result<Self> {
        let Some(selected) = self.table_state.selected() else {
            return Ok(self);
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

        Ok(self.with_updated_rows())
    }

    pub fn selected_to_current_exercise(&mut self) -> Result<()> {
        let Some(selected) = self.table_state.selected() else {
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

        self.app_state.set_current_exercise_ind(ind)
    }
}
