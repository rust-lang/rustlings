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
    // Only send `Unrecognized` on ENTER if the last input wasn't valid.
    let mut last_input_valid = false;

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
                    last_input_valid = false;
                    continue;
                }

                let input_event = match key.code {
                    KeyCode::Enter => {
                        if last_input_valid {
                            continue;
                        }

                        InputEvent::Unrecognized
                    }
                    KeyCode::Char(c) => {
                        let input_event = match c {
                            'n' => InputEvent::Next,
                            'h' => InputEvent::Hint,
                            'l' => break InputEvent::List,
                            'q' => break InputEvent::Quit,
                            'r' if manual_run => InputEvent::Run,
                            _ => {
                                last_input_valid = false;
                                continue;
                            }
                        };

                        last_input_valid = true;
                        input_event
                    }
                    _ => {
                        last_input_valid = false;
                        continue;
                    }
                };

                if tx.send(WatchEvent::Input(input_event)).is_err() {
                    return;
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
