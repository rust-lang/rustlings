use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use std::sync::mpsc::Sender;

use super::WatchEvent;

pub struct DebounceEventHandler {
    pub tx: Sender<WatchEvent>,
    pub exercise_names: &'static [&'static [u8]],
}

impl notify_debouncer_mini::DebounceEventHandler for DebounceEventHandler {
    fn handle_event(&mut self, event: DebounceEventResult) {
        let event = match event {
            Ok(event) => {
                let Some(exercise_ind) = event
                    .iter()
                    .filter_map(|event| {
                        if event.kind != DebouncedEventKind::Any {
                            return None;
                        }

                        let file_name = event.path.file_name()?.to_str()?.as_bytes();

                        if file_name.len() < 4 {
                            return None;
                        }
                        let (file_name_without_ext, ext) = file_name.split_at(file_name.len() - 3);

                        if ext != b".rs" {
                            return None;
                        }

                        self.exercise_names
                            .iter()
                            .position(|exercise_name| *exercise_name == file_name_without_ext)
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
