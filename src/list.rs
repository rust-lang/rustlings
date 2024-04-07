use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod state;

use crate::{exercise::Exercise, state_file::StateFile};

use self::state::UiState;

pub fn list(state_file: &mut StateFile, exercises: &[Exercise]) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    let mut ui_state = UiState::new(state_file, exercises);

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
                state_file.set_next_exercise_ind(ui_state.selected())?;
                ui_state.table = ui_state.table.rows(UiState::rows(state_file, exercises));
            }
            _ => (),
        }
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
