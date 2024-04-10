use anyhow::{Error, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use notify_debouncer_mini::{
    new_debouncer,
    notify::{self, RecursiveMode},
    DebounceEventResult, DebouncedEventKind,
};
use std::{
    io::{self, Write},
    path::Path,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

mod state;

use crate::{exercise::Exercise, state_file::StateFile};

use self::state::WatchState;

/// Returned by the watch mode to indicate what to do afterwards.
pub enum WatchExit {
    /// Exit the program.
    Shutdown,
    /// Enter the list mode and restart the watch mode afterwards.
    List,
}

#[derive(Copy, Clone)]
enum InputEvent {
    Hint,
    Clear,
    List,
    Quit,
    Unrecognized,
}

enum WatchEvent {
    Input(InputEvent),
    FileChange { exercise_ind: usize },
    NotifyErr(notify::Error),
    TerminalEventErr(io::Error),
    TerminalResize,
}

struct DebouceEventHandler {
    tx: Sender<WatchEvent>,
    exercises: &'static [Exercise],
}

impl notify_debouncer_mini::DebounceEventHandler for DebouceEventHandler {
    fn handle_event(&mut self, event: DebounceEventResult) {
        let event = match event {
            Ok(event) => {
                let Some(exercise_ind) = event
                    .iter()
                    .filter_map(|event| {
                        if event.kind != DebouncedEventKind::Any
                            || !event.path.extension().is_some_and(|ext| ext == "rs")
                        {
                            return None;
                        }

                        self.exercises
                            .iter()
                            .position(|exercise| event.path.ends_with(&exercise.path))
                    })
                    .min()
                else {
                    return;
                };

                WatchEvent::FileChange { exercise_ind }
            }
            Err(e) => WatchEvent::NotifyErr(e),
        };

        // An error occurs when the receiver is dropped.
        // After dropping the receiver, the debouncer guard should also be dropped.
        let _ = self.tx.send(event);
    }
}

fn terminal_event_handler(tx: Sender<WatchEvent>) {
    let mut input = String::with_capacity(8);

    loop {
        let terminal_event = match event::read() {
            Ok(v) => v,
            Err(e) => {
                // If `send` returns an error, then the receiver is dropped and
                // a shutdown has been already initialized.
                let _ = tx.send(WatchEvent::TerminalEventErr(e));
                return;
            }
        };

        match terminal_event {
            Event::Key(key) => {
                match key.kind {
                    KeyEventKind::Release => continue,
                    KeyEventKind::Press | KeyEventKind::Repeat => (),
                }

                match key.code {
                    KeyCode::Enter => {
                        let input_event = match input.trim() {
                            "h" | "hint" => InputEvent::Hint,
                            "c" | "clear" => InputEvent::Clear,
                            "l" | "list" => InputEvent::List,
                            "q" | "quit" => InputEvent::Quit,
                            _ => InputEvent::Unrecognized,
                        };

                        if tx.send(WatchEvent::Input(input_event)).is_err() {
                            return;
                        }

                        match input_event {
                            InputEvent::List | InputEvent::Quit => return,
                            _ => (),
                        }

                        input.clear();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    _ => (),
                }
            }
            Event::Resize(_, _) => {
                if tx.send(WatchEvent::TerminalResize).is_err() {
                    return;
                }
            }
            Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => continue,
        }
    }
}

pub fn watch(state_file: &mut StateFile, exercises: &'static [Exercise]) -> Result<WatchExit> {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        DebouceEventHandler {
            tx: tx.clone(),
            exercises,
        },
    )?;
    debouncer
        .watcher()
        .watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    let mut watch_state = WatchState::new(state_file, exercises);

    // TODO: bool
    watch_state.run_exercise()?;
    watch_state.render()?;

    thread::spawn(move || terminal_event_handler(tx));

    while let Ok(event) = rx.recv() {
        match event {
            WatchEvent::Input(InputEvent::Hint) => {
                watch_state.show_hint()?;
            }
            WatchEvent::Input(InputEvent::List) => {
                return Ok(WatchExit::List);
            }
            WatchEvent::Input(InputEvent::Clear) | WatchEvent::TerminalResize => {
                watch_state.render()?;
            }
            WatchEvent::Input(InputEvent::Quit) => break,
            WatchEvent::Input(InputEvent::Unrecognized) => {
                watch_state.handle_invalid_cmd()?;
            }
            WatchEvent::FileChange { exercise_ind } => {
                // TODO: bool
                watch_state.run_exercise_with_ind(exercise_ind)?;
                watch_state.render()?;
            }
            WatchEvent::NotifyErr(e) => {
                return Err(Error::from(e).context("Exercise file watcher failed"))
            }
            WatchEvent::TerminalEventErr(e) => {
                return Err(Error::from(e).context("Terminal event listener failed"))
            }
        }
    }

    watch_state.into_writer().write_all(b"
We hope you're enjoying learning Rust!
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again.
")?;

    Ok(WatchExit::Shutdown)
}
