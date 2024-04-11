use anyhow::{Error, Result};
use notify_debouncer_mini::{
    new_debouncer,
    notify::{self, RecursiveMode},
};
use std::{
    io::{self, Write},
    path::Path,
    sync::mpsc::channel,
    thread,
    time::Duration,
};

mod debounce_event;
mod state;
mod terminal_event;

use crate::app_state::AppState;

use self::{
    debounce_event::DebounceEventHandler,
    state::WatchState,
    terminal_event::{terminal_event_handler, InputEvent},
};

enum WatchEvent {
    Input(InputEvent),
    FileChange { exercise_ind: usize },
    NotifyErr(notify::Error),
    TerminalEventErr(io::Error),
    TerminalResize,
}

/// Returned by the watch mode to indicate what to do afterwards.
pub enum WatchExit {
    /// Exit the program.
    Shutdown,
    /// Enter the list mode and restart the watch mode afterwards.
    List,
}

pub fn watch(app_state: &mut AppState) -> Result<WatchExit> {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        DebounceEventHandler {
            tx: tx.clone(),
            exercises: app_state.exercises(),
        },
    )?;
    debouncer
        .watcher()
        .watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    let mut watch_state = WatchState::new(app_state);

    // TODO: bool
    watch_state.run_current_exercise()?;
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
            WatchEvent::TerminalResize => {
                watch_state.render()?;
            }
            WatchEvent::Input(InputEvent::Quit) => break,
            WatchEvent::Input(InputEvent::Unrecognized(cmd)) => {
                watch_state.handle_invalid_cmd(&cmd)?;
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
