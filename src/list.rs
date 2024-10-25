use anyhow::{Context, Result};
use crossterm::{
    cursor,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};
use std::io::{self, StdoutLock, Write};

use crate::app_state::AppState;

use self::state::{Filter, ListState};

mod scroll_state;
mod state;

fn handle_list(app_state: &mut AppState, stdout: &mut StdoutLock) -> Result<()> {
    let mut list_state = ListState::build(app_state, stdout)?;
    let mut is_searching = false;

    loop {
        match event::read().context("Failed to read terminal event")? {
            Event::Key(key) => {
                match key.kind {
                    KeyEventKind::Release => continue,
                    KeyEventKind::Press | KeyEventKind::Repeat => (),
                }

                list_state.message.clear();

                if is_searching {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter => {
                            is_searching = false;
                            list_state.search_query.clear();
                        }
                        KeyCode::Char(c) => {
                            list_state.search_query.push(c);
                            list_state.apply_search_query();
                        }
                        KeyCode::Backspace => {
                            list_state.search_query.pop();
                            list_state.apply_search_query();
                        }
                        _ => continue,
                    }

                    list_state.draw(stdout)?;
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => list_state.select_next(),
                    KeyCode::Up | KeyCode::Char('k') => list_state.select_previous(),
                    KeyCode::Home | KeyCode::Char('g') => list_state.select_first(),
                    KeyCode::End | KeyCode::Char('G') => list_state.select_last(),
                    KeyCode::Char('d') => {
                        if list_state.filter() == Filter::Done {
                            list_state.set_filter(Filter::None);
                            list_state.message.push_str("Disabled filter DONE");
                        } else {
                            list_state.set_filter(Filter::Done);
                            list_state.message.push_str(
                                "Enabled filter DONE │ Press d again to disable the filter",
                            );
                        }
                    }
                    KeyCode::Char('p') => {
                        if list_state.filter() == Filter::Pending {
                            list_state.set_filter(Filter::None);
                            list_state.message.push_str("Disabled filter PENDING");
                        } else {
                            list_state.set_filter(Filter::Pending);
                            list_state.message.push_str(
                                "Enabled filter PENDING │ Press p again to disable the filter",
                            );
                        }
                    }
                    KeyCode::Char('r') => list_state.reset_selected()?,
                    KeyCode::Char('c') => {
                        if list_state.selected_to_current_exercise()? {
                            return Ok(());
                        }
                    }
                    KeyCode::Char('s' | '/') => {
                        is_searching = true;
                        list_state.apply_search_query();
                    }
                    // Redraw to remove the message.
                    KeyCode::Esc => (),
                    _ => continue,
                }
            }
            Event::Mouse(event) => match event.kind {
                MouseEventKind::ScrollDown => list_state.select_next(),
                MouseEventKind::ScrollUp => list_state.select_previous(),
                _ => continue,
            },
            Event::Resize(width, height) => list_state.set_term_size(width, height),
            // Ignore
            Event::FocusGained | Event::FocusLost => continue,
        }

        list_state.draw(stdout)?;
    }
}

pub fn list(app_state: &mut AppState) -> Result<()> {
    let mut stdout = io::stdout().lock();
    stdout
        .queue(EnterAlternateScreen)?
        .queue(cursor::Hide)?
        .queue(DisableLineWrap)?
        .queue(EnableMouseCapture)?;
    enable_raw_mode()?;

    let res = handle_list(app_state, &mut stdout);

    // Restore the terminal even if we got an error.
    stdout
        .queue(LeaveAlternateScreen)?
        .queue(cursor::Show)?
        .queue(EnableLineWrap)?
        .queue(DisableMouseCapture)?
        .flush()?;
    disable_raw_mode()?;

    res
}
