use notify_debouncer_mini::{DebounceEventResult, DebouncedEventKind};
use std::sync::mpsc::Sender;

use super::WatchEvent;

pub struct NotifyEventHandler {
    pub tx: Sender<WatchEvent>,
    /// Used to report which exercise was modified.
    pub exercise_names: &'static [&'static [u8]],
}

impl notify_debouncer_mini::DebounceEventHandler for NotifyEventHandler {
    fn handle_event(&mut self, input_event: DebounceEventResult) {
        let output_event = match input_event {
            Ok(input_event) => {
                let Some(exercise_ind) = input_event
                    .iter()
                    .filter_map(|input_event| {
                        if input_event.kind != DebouncedEventKind::Any {
                            return None;
                        }

                        let file_name = input_event.path.file_name()?.to_str()?.as_bytes();

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
        let _ = self.tx.send(output_event);
    }
}
