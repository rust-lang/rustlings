use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    mpsc::Sender,
};

use super::WatchEvent;

static INPUT_PAUSED: AtomicBool = AtomicBool::new(false);

// Private unit type to force using the constructor function.
#[must_use = "When the guard is dropped, the input is unpaused"]
pub struct InputPauseGuard(());

impl InputPauseGuard {
    #[inline]
    pub fn scoped_pause() -> Self {
        INPUT_PAUSED.store(true, Relaxed);
        Self(())
    }
}

impl Drop for InputPauseGuard {
    #[inline]
    fn drop(&mut self) {
        INPUT_PAUSED.store(false, Relaxed);
    }
}

pub enum InputEvent {
    Run,
    Next,
    Hint,
    List,
    Quit,
}

pub fn terminal_event_handler(sender: Sender<WatchEvent>, manual_run: bool) {
    let last_watch_event = loop {
        match event::read() {
            Ok(Event::Key(key)) => {
                match key.kind {
                    KeyEventKind::Release | KeyEventKind::Repeat => continue,
                    KeyEventKind::Press => (),
                }

                if INPUT_PAUSED.load(Relaxed) {
                    continue;
                }

                let input_event = match key.code {
                    KeyCode::Char('n') => InputEvent::Next,
                    KeyCode::Char('h') => InputEvent::Hint,
                    KeyCode::Char('l') => break WatchEvent::Input(InputEvent::List),
                    KeyCode::Char('q') => break WatchEvent::Input(InputEvent::Quit),
                    KeyCode::Char('r') if manual_run => InputEvent::Run,
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
