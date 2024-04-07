use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Row, Table, TableState},
    Frame, Terminal,
};
use std::io;

use crate::{exercise::Exercise, state::State};

struct UiState<'a> {
    pub table: Table<'a>,
    selected: usize,
    table_state: TableState,
    last_ind: usize,
}

impl<'a> UiState<'a> {
    pub fn rows<'s, 'i>(
        state: &'s State,
        exercises: &'a [Exercise],
    ) -> impl Iterator<Item = Row<'a>> + 'i
    where
        's: 'i,
        'a: 'i,
    {
        exercises
            .iter()
            .zip(state.progress())
            .enumerate()
            .map(|(ind, (exercise, done))| {
                let next = if ind == state.next_exercise_ind() {
                    ">>>>".bold().red()
                } else {
                    Span::default()
                };

                let exercise_state = if *done {
                    "DONE".green()
                } else {
                    "PENDING".yellow()
                };

                Row::new([
                    next,
                    exercise_state,
                    Span::raw(&exercise.name),
                    Span::raw(exercise.path.to_string_lossy()),
                ])
            })
    }

    pub fn new(state: &State, exercises: &'a [Exercise]) -> Self {
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

        let rows = Self::rows(state, exercises);

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(2)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::new().bg(ratatui::style::Color::Rgb(50, 50, 50)))
            .highlight_symbol("🦀")
            .block(Block::default().borders(Borders::BOTTOM));

        let selected = 0;
        let table_state = TableState::default().with_selected(Some(selected));
        let last_ind = exercises.len() - 1;

        Self {
            table,
            selected,
            table_state,
            last_ind,
        }
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

        // Help footer
        let footer =
            "↓/j ↑/k home/g end/G │ Filter <d>one/<p>ending │ <r>eset │ <c>ontinue at │ <q>uit";
        frame.render_widget(
            Span::raw(footer),
            Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            },
        );
    }
}

pub fn list(state: &mut State, exercises: &[Exercise]) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    let mut ui_state = UiState::new(state, exercises);

    'outer: loop {
        terminal.draw(|frame| ui_state.draw(frame))?;

        let key = loop {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    break key;
                }
                // Redraw
                Event::Resize(_, _) => continue 'outer,
                // Ignore
                Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => (),
            }
        };

        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Down | KeyCode::Char('j') => ui_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => ui_state.select_previous(),
            KeyCode::Home | KeyCode::Char('g') => ui_state.select_first(),
            KeyCode::End | KeyCode::Char('G') => ui_state.select_last(),
            KeyCode::Char('c') => {
                state.set_next_exercise_ind(ui_state.selected)?;
                ui_state.table = ui_state.table.rows(UiState::rows(state, exercises));
            }
            _ => (),
        }
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
