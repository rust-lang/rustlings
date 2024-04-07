use anyhow::Result;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::{
    io::{self, BufRead, Write},
    path::Path,
    sync::mpsc::{channel, sync_channel},
    thread,
    time::Duration,
};

mod state;

use crate::{exercise::Exercise, state_file::StateFile};

use self::state::WatchState;

enum Event {
    Hint,
    Clear,
    Quit,
}

pub fn watch(state_file: &StateFile, exercises: &[Exercise]) -> Result<()> {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    let mut watch_state = WatchState::new(state_file, exercises, rx);

    watch_state.run_exercise()?;
    watch_state.render()?;

    let (tx, rx) = sync_channel(0);
    thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let mut stdin_buf = String::with_capacity(8);

        loop {
            stdin.read_line(&mut stdin_buf).unwrap();

            let event = match stdin_buf.trim() {
                "h" | "hint" => Some(Event::Hint),
                "c" | "clear" => Some(Event::Clear),
                "q" | "quit" => Some(Event::Quit),
                _ => None,
            };

            stdin_buf.clear();

            if tx.send(event).is_err() {
                break;
            };
        }
    });

    loop {
        watch_state.try_recv_event()?;

        if let Ok(event) = rx.try_recv() {
            match event {
                Some(Event::Hint) => {
                    watch_state.show_hint()?;
                }
                Some(Event::Clear) => {
                    watch_state.render()?;
                }
                Some(Event::Quit) => break,
                None => {
                    watch_state.handle_invalid_cmd()?;
                }
            }
        }
    }

    watch_state.into_writer().write_all(b"
We hope you're enjoying learning Rust!
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again.
")?;

    Ok(())
}
