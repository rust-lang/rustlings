use notify::{
    event::{MetadataKind, ModifyKind},
    Event, EventKind,
};
use std::sync::{atomic::Ordering::Relaxed, mpsc::Sender};

use super::{WatchEvent, EXERCISE_RUNNING};

pub struct NotifyEventHandler {
    pub sender: Sender<WatchEvent>,
    /// Used to report which exercise was modified.
    pub exercise_names: &'static [&'static [u8]],
}

impl notify::EventHandler for NotifyEventHandler {
    fn handle_event(&mut self, input_event: notify::Result<Event>) {
        if EXERCISE_RUNNING.load(Relaxed) {
            return;
        }

        let input_event = match input_event {
            Ok(v) => v,
            Err(e) => {
                // An error occurs when the receiver is dropped.
                // After dropping the receiver, the debouncer guard should also be dropped.
                let _ = self.sender.send(WatchEvent::NotifyErr(e));
                return;
            }
        };

        match input_event.kind {
            EventKind::Any => (),
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Any | ModifyKind::Data(_) => (),
                ModifyKind::Metadata(metadata_kind) => match metadata_kind {
                    MetadataKind::Any | MetadataKind::WriteTime => (),
                    MetadataKind::AccessTime
                    | MetadataKind::Permissions
                    | MetadataKind::Ownership
                    | MetadataKind::Extended
                    | MetadataKind::Other => return,
                },
                ModifyKind::Name(_) | ModifyKind::Other => return,
            },
            EventKind::Access(_)
            | EventKind::Create(_)
            | EventKind::Remove(_)
            | EventKind::Other => return,
        }

        let _ = input_event
            .paths
            .into_iter()
            .filter_map(|path| {
                let file_name = path.file_name()?.to_str()?.as_bytes();

                let [file_name_without_ext @ .., b'.', b'r', b's'] = file_name else {
                    return None;
                };

                self.exercise_names
                    .iter()
                    .position(|exercise_name| *exercise_name == file_name_without_ext)
            })
            .try_for_each(|exercise_ind| self.sender.send(WatchEvent::FileChange { exercise_ind }));
    }
}
