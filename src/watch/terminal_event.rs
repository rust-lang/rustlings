use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::sync::mpsc::Sender;

use super::WatchEvent;

pub enum InputEvent {
    Run,
    Next,
    Hint,
    List,
    Quit,
    Unrecognized,
}

pub fn terminal_event_handler(tx: Sender<WatchEvent>, manual_run: bool) {
    let last_input_event = loop {
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
                    KeyEventKind::Release | KeyEventKind::Repeat => continue,
                    KeyEventKind::Press => (),
                }

                if key.modifiers != KeyModifiers::NONE {
                    continue;
                }

                if let KeyCode::Char(c) = key.code {
                    let input_event = match c {
                        'n' => InputEvent::Next,
                        'h' => InputEvent::Hint,
                        'l' => break InputEvent::List,
                        'q' => break InputEvent::Quit,
                        'r' if manual_run => InputEvent::Run,
                        _ => InputEvent::Unrecognized,
                    };

                    if tx.send(WatchEvent::Input(input_event)).is_err() {
                        return;
                    }
                }
            }
            Event::Resize(_, _) => {
                if tx.send(WatchEvent::TerminalResize).is_err() {
                    return;
                }
            }
            Event::FocusGained | Event::FocusLost | Event::Mouse(_) | Event::Paste(_) => continue,
        }
    };

    let _ = tx.send(WatchEvent::Input(last_input_event));
}
