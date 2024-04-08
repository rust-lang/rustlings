use ratatui::{
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Row, Table, TableState},
    Frame,
};

use crate::{exercise::Exercise, state_file::StateFile};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Filter {
    Done,
    Pending,
    None,
}

pub struct UiState<'a> {
    pub table: Table<'a>,
    pub message: String,
    pub filter: Filter,
    exercises: &'a [Exercise],
    selected: usize,
    table_state: TableState,
    last_ind: usize,
}

impl<'a> UiState<'a> {
    fn rows<'s, 'c, 'i>(
        state_file: &'s StateFile,
        exercises: &'a [Exercise],
        rows_counter: &'c mut usize,
        filter: Filter,
    ) -> impl Iterator<Item = Row<'a>> + 'i
    where
        's: 'i,
        'a: 'i,
        'c: 'i,
    {
        exercises
            .iter()
            .zip(state_file.progress().iter().copied())
            .enumerate()
            .filter_map(move |(ind, (exercise, done))| {
                match (filter, done) {
                    (Filter::Done, false) | (Filter::Pending, true) => return None,
                    _ => (),
                }

                *rows_counter += 1;

                let next = if ind == state_file.next_exercise_ind() {
                    ">>>>".bold().red()
                } else {
                    Span::default()
                };

                let exercise_state = if done {
                    "DONE".green()
                } else {
                    "PENDING".yellow()
                };

                Some(Row::new([
                    next,
                    exercise_state,
                    Span::raw(&exercise.name),
                    Span::raw(exercise.path.to_string_lossy()),
                ]))
            })
    }

    pub fn with_updated_rows(mut self, state_file: &StateFile) -> Self {
        let mut rows_counter = 0;
        let rows = Self::rows(state_file, self.exercises, &mut rows_counter, self.filter);
        self.table = self.table.rows(rows);

        self.last_ind = rows_counter.saturating_sub(1);
        self.select(self.selected.min(self.last_ind));

        self
    }

    pub fn new(state_file: &StateFile, exercises: &'a [Exercise]) -> Self {
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

        let filter = Filter::None;
        let mut rows_counter = 0;
        let rows = Self::rows(state_file, exercises, &mut rows_counter, filter);

        let table = Table::new(rows, widths)
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

        Self {
            table,
            message: String::with_capacity(128),
            filter,
            exercises,
            selected,
            table_state,
            last_ind: rows_counter.saturating_sub(1),
        }
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

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.size();

        frame.render_stateful_widget(
            &self.table,
            Rect {
                x: 0,
                y: 0,
                width: area.width,
                height: area.height - 1,
            },
            &mut self.table_state,
        );

        let message = if self.message.is_empty() {
            // Help footer.
            "↓/j ↑/k home/g end/G │ filter <d>one/<p>ending │ <r>eset │ <c>ontinue at │ <q>uit"
        } else {
            &self.message
        };
        frame.render_widget(
            Span::raw(message),
            Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            },
        );
    }
}
