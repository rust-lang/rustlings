use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::mpsc::Sender;

use super::WatchEvent;

pub enum InputEvent {
    Run,
    Next,
    Hint,
    List,
    Quit,
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

                let input_event = match key.code {
                    KeyCode::Char('n') => InputEvent::Next,
                    KeyCode::Char('h') => InputEvent::Hint,
                    KeyCode::Char('l') => break InputEvent::List,
                    KeyCode::Char('q') => break InputEvent::Quit,
                    KeyCode::Char('r') if manual_run => InputEvent::Run,
                    _ => continue,
                };

                if tx.send(WatchEvent::Input(input_event)).is_err() {
                    return;
                }
            }
            Event::Resize(width, _) => {
                if tx.send(WatchEvent::TerminalResize { width }).is_err() {
                    return;
                }
            }
            Event::FocusGained | Event::FocusLost | Event::Mouse(_) => continue,
        }
    };

    let _ = tx.send(WatchEvent::Input(last_input_event));
}
