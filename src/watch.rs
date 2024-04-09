use anyhow::Result;
use notify_debouncer_mini::{
    new_debouncer,
    notify::{self, RecursiveMode},
    DebounceEventResult, DebouncedEventKind,
};
use std::{
    io::{self, BufRead, Write},
    path::Path,
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

mod state;

use crate::{exercise::Exercise, state_file::StateFile};

use self::state::WatchState;

enum InputEvent {
    Hint,
    Clear,
    Quit,
    Unrecognized,
}

enum WatchEvent {
    Input(InputEvent),
    FileChange { exercise_ind: usize },
    NotifyErr(notify::Error),
    StdinErr(io::Error),
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

fn input_handler(tx: Sender<WatchEvent>) {
    let mut stdin = io::stdin().lock();
    let mut stdin_buf = String::with_capacity(8);

    loop {
        if let Err(e) = stdin.read_line(&mut stdin_buf) {
            // If `send` returns an error, then the receiver is dropped and
            // a shutdown has been already initialized.
            let _ = tx.send(WatchEvent::StdinErr(e));
            return;
        }

        let event = match stdin_buf.trim() {
            "h" | "hint" => InputEvent::Hint,
            "c" | "clear" => InputEvent::Clear,
            "q" | "quit" => InputEvent::Quit,
            _ => InputEvent::Unrecognized,
        };

        stdin_buf.clear();

        if tx.send(WatchEvent::Input(event)).is_err() {
            // The receiver was dropped.
            return;
        }
    }
}

pub fn watch(state_file: &StateFile, exercises: &'static [Exercise]) -> Result<()> {
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

    thread::spawn(move || input_handler(tx));

    while let Ok(event) = rx.recv() {
        match event {
            WatchEvent::Input(InputEvent::Hint) => {
                watch_state.show_hint()?;
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
            WatchEvent::NotifyErr(e) => return Err(e.into()),
            WatchEvent::StdinErr(e) => return Err(e.into()),
        }
    }

    watch_state.into_writer().write_all(b"
We hope you're enjoying learning Rust!
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again.
")?;

    Ok(())
}
