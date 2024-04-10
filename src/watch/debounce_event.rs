use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use std::sync::mpsc::Sender;

use crate::exercise::Exercise;

use super::WatchEvent;

pub struct DebounceEventHandler {
    pub tx: Sender<WatchEvent>,
    pub exercises: &'static [Exercise],
}

impl notify_debouncer_mini::DebounceEventHandler for DebounceEventHandler {
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
