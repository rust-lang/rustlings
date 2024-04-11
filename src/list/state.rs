use anyhow::Result;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{app_state::AppState, exercise::Exercise, progress_bar::progress_bar_ratatui};

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
    selected: usize,
    last_ind: usize,
}

impl<'a> UiState<'a> {
    pub fn with_updated_rows(mut self) -> Self {
        let current_exercise_ind = self.app_state.current_exercise_ind();

        let mut rows_counter: usize = 0;
        let rows = self
            .app_state
            .exercises()
            .iter()
            .zip(self.app_state.progress().iter().copied())
            .enumerate()
            .filter_map(|(ind, (exercise, done))| {
                let exercise_state = if done {
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

                rows_counter += 1;

                let next = if ind == current_exercise_ind {
                    ">>>>".bold().red()
                } else {
                    Span::default()
                };

                Some(Row::new([
                    next,
                    exercise_state,
                    Span::raw(&exercise.name),
                    Span::raw(exercise.path.to_string_lossy()),
                ]))
            });

        self.table = self.table.rows(rows);

        self.last_ind = rows_counter.saturating_sub(1);
        self.select(self.selected.min(self.last_ind));

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

        let slf = Self {
            table,
            message: String::with_capacity(128),
            filter: Filter::None,
            app_state,
            table_state,
            selected,
            last_ind: 0,
        };

        slf.with_updated_rows()
    }

    fn select(&mut self, ind: usize) {
        self.selected = ind;
        self.table_state.select(Some(ind));
    }

    pub fn select_next(&mut self) {
        let next = (self.selected + 1).min(self.last_ind);
        self.select(next);
    }

    pub fn select_previous(&mut self) {
        let previous = self.selected.saturating_sub(1);
        self.select(previous);
    }

    #[inline]
    pub fn select_first(&mut self) {
        self.select(0);
    }

    #[inline]
    pub fn select_last(&mut self) {
        self.select(self.last_ind);
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
            Span::raw(
                "↓/j ↑/k home/g end/G │ filter <d>one/<p>ending │ <r>eset │ <c>ontinue at │ <q>uit",
            )
        } else {
            self.message.as_str().light_blue()
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

    pub fn reset_selected(&mut self) -> Result<&'static Exercise> {
        self.app_state.set_pending(self.selected)?;
        // TODO: Take care of filters!
        let exercise = &self.app_state.exercises()[self.selected];
        exercise.reset()?;

        Ok(exercise)
    }

    #[inline]
    pub fn selected_to_current_exercise(&mut self) -> Result<()> {
        // TODO: Take care of filters!
        self.app_state.set_current_exercise_ind(self.selected)
    }
}
