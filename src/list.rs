use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, Row, Table, TableState},
    Terminal,
};

use crate::{exercise::Exercise, state::State};

// 40 FPS.
const UPDATE_INTERVAL: Duration = Duration::from_millis(25);

pub fn list(state: &State, exercises: &[Exercise]) -> Result<()> {
    let mut stdout = io::stdout().lock();

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    let header = Row::new(["State", "Name", "Path"]);

    let max_name_len = exercises
        .iter()
        .map(|exercise| exercise.name.len())
        .max()
        .unwrap_or(4) as u16;

    let widths = [
        Constraint::Length(7),
        Constraint::Length(max_name_len),
        Constraint::Fill(1),
    ];

    let rows = exercises
        .iter()
        .zip(&state.progress)
        .map(|(exercise, done)| {
            let state = if *done {
                "DONE".green()
            } else {
                "PENDING".yellow()
            };
            Row::new([
                state,
                Span::raw(&exercise.name),
                Span::raw(exercise.path.to_string_lossy()),
            ])
        })
        .collect::<Vec<_>>();

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(2)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("🦀");

    let mut table_state = TableState::default().with_selected(Some(0));

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_stateful_widget(&table, area, &mut table_state);
        })?;

        if event::poll(UPDATE_INTERVAL)? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
