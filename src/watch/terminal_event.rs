use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::sync::mpsc::Sender;

use super::WatchEvent;

pub enum InputEvent {
    Next,
    Hint,
    List,
    Quit,
    Unrecognized(String),
}

pub fn terminal_event_handler(tx: Sender<WatchEvent>) {
    let mut input = String::with_capacity(8);

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
                if key.modifiers != KeyModifiers::NONE {
                    continue;
                }

                match key.kind {
                    KeyEventKind::Release => continue,
                    KeyEventKind::Press | KeyEventKind::Repeat => (),
                }

                match key.code {
                    KeyCode::Enter => {
                        let input_event = match input.trim() {
                            "n" | "next" => InputEvent::Next,
                            "h" | "hint" => InputEvent::Hint,
                            "l" | "list" => break InputEvent::List,
                            "q" | "quit" => break InputEvent::Quit,
                            _ => InputEvent::Unrecognized(input.clone()),
                        };

                        if tx.send(WatchEvent::Input(input_event)).is_err() {
                            return;
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
    };

    let _ = tx.send(WatchEvent::Input(last_input_event));
}
