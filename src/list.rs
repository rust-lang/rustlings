use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{fmt::Write, io};

mod state;

use crate::{exercise::Exercise, state_file::StateFile};

use self::state::{Filter, UiState};

pub fn list(state_file: &mut StateFile, exercises: &'static [Exercise]) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    let mut ui_state = UiState::new(state_file, exercises);

    'outer: loop {
        terminal.draw(|frame| ui_state.draw(frame).unwrap())?;

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

        ui_state.message.clear();

        match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Down | KeyCode::Char('j') => ui_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => ui_state.select_previous(),
            KeyCode::Home | KeyCode::Char('g') => ui_state.select_first(),
            KeyCode::End | KeyCode::Char('G') => ui_state.select_last(),
            KeyCode::Char('d') => {
                let message = if ui_state.filter == Filter::Done {
                    ui_state.filter = Filter::None;
                    "Disabled filter DONE"
                } else {
                    ui_state.filter = Filter::Done;
                    "Enabled filter DONE │ Press d again to disable the filter"
                };

                ui_state = ui_state.with_updated_rows(state_file);
                ui_state.message.push_str(message);
            }
            KeyCode::Char('p') => {
                let message = if ui_state.filter == Filter::Pending {
                    ui_state.filter = Filter::None;
                    "Disabled filter PENDING"
                } else {
                    ui_state.filter = Filter::Pending;
                    "Enabled filter PENDING │ Press p again to disable the filter"
                };

                ui_state = ui_state.with_updated_rows(state_file);
                ui_state.message.push_str(message);
            }
            KeyCode::Char('r') => {
                let selected = ui_state.selected();
                let exercise = &exercises[selected];
                exercise.reset()?;
                state_file.reset(selected)?;

                ui_state = ui_state.with_updated_rows(state_file);
                ui_state
                    .message
                    .write_fmt(format_args!("The exercise {exercise} has been reset!"))?;
            }
            KeyCode::Char('c') => {
                state_file.set_next_exercise_ind(ui_state.selected())?;
                ui_state = ui_state.with_updated_rows(state_file);
            }
            _ => (),
        }
    }

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
