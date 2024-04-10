use anyhow::Result;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{exercise::Exercise, progress_bar::progress_bar_ratatui, state_file::StateFile};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    Done,
    Pending,
    None,
}

pub struct UiState {
    pub table: Table<'static>,
    pub message: String,
    pub filter: Filter,
    exercises: &'static [Exercise],
    progress: u16,
    selected: usize,
    table_state: TableState,
    last_ind: usize,
}

impl UiState {
    pub fn with_updated_rows(mut self, state_file: &StateFile) -> Self {
        let mut rows_counter: usize = 0;
        let mut progress: u16 = 0;
        let rows = self
            .exercises
            .iter()
            .zip(state_file.progress().iter().copied())
            .enumerate()
            .filter_map(|(ind, (exercise, done))| {
                let exercise_state = if done {
                    progress += 1;

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

                let next = if ind == state_file.next_exercise_ind() {
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

        self.progress = progress;

        self
    }

    pub fn new(state_file: &StateFile, exercises: &'static [Exercise]) -> Self {
        let header = Row::new(["Next", "State", "Name", "Path"]);

        let max_name_len = exercises
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

        let selected = state_file.next_exercise_ind();
        let table_state = TableState::default()
            .with_offset(selected.saturating_sub(10))
            .with_selected(Some(selected));

        let slf = Self {
            table,
            message: String::with_capacity(128),
            filter: Filter::None,
            exercises,
            progress: 0,
            selected,
            table_state,
            last_ind: 0,
        };

        slf.with_updated_rows(state_file)
    }

    #[inline]
    pub fn selected(&self) -> usize {
        self.selected
    }

    fn select(&mut self, ind: usize) {
        self.selected = ind;
        self.table_state.select(Some(ind));
    }

    pub fn select_next(&mut self) {
        self.select(self.selected.saturating_add(1).min(self.last_ind));
    }

    pub fn select_previous(&mut self) {
        self.select(self.selected.saturating_sub(1));
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
                self.progress,
                self.exercises.len() as u16,
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
}
