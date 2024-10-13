use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::{
    atomic::Ordering::Relaxed,
    mpsc::{Receiver, Sender},
};

use super::{WatchEvent, EXERCISE_RUNNING};

pub enum InputEvent {
    Next,
    Run,
    Hint,
    List,
    CheckAll,
    Reset,
    Quit,
}

pub fn terminal_event_handler(
    sender: Sender<WatchEvent>,
    unpause_receiver: Receiver<()>,
    manual_run: bool,
) {
    let last_watch_event = loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                match key.kind {
                    KeyEventKind::Release | KeyEventKind::Repeat => continue,
                    KeyEventKind::Press => (),
                }

                if EXERCISE_RUNNING.load(Relaxed) {
                    continue;
                }

                let input_event = match key.code {
                    KeyCode::Char('n') => InputEvent::Next,
                    KeyCode::Char('r') if manual_run => InputEvent::Run,
                    KeyCode::Char('h') => InputEvent::Hint,
                    KeyCode::Char('l') => break WatchEvent::Input(InputEvent::List),
                    KeyCode::Char('c') => InputEvent::CheckAll,
                    KeyCode::Char('x') => {
                        if sender.send(WatchEvent::Input(InputEvent::Reset)).is_err() {
                            return;
                        }

                        // Pause input until quitting the confirmation prompt.
                        if unpause_receiver.recv().is_err() {
                            return;
                        };

                        continue;
                    }
                    KeyCode::Char('q') => break WatchEvent::Input(InputEvent::Quit),
                    _ => continue,
                };

                if sender.send(WatchEvent::Input(input_event)).is_err() {
                    return;
                }
            }
            Ok(Event::Resize(width, _)) => {
                if sender.send(WatchEvent::TerminalResize { width }).is_err() {
                    return;
                }
            }
            Ok(Event::FocusGained | Event::FocusLost | Event::Mouse(_)) => continue,
            Err(e) => break WatchEvent::TerminalEventErr(e),
        }
    };

    let _ = sender.send(last_watch_event);
}
