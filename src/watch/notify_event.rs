use anyhow::{Context, Result};
use notify::{
    event::{AccessKind, AccessMode, MetadataKind, ModifyKind, RenameMode},
    Event, EventKind,
};
use std::{
    sync::{
        atomic::Ordering::Relaxed,
        mpsc::{sync_channel, RecvTimeoutError, Sender, SyncSender},
    },
    thread,
    time::Duration,
};

use super::{WatchEvent, EXERCISE_RUNNING};

const DEBOUNCE_DURATION: Duration = Duration::from_millis(200);

pub struct NotifyEventHandler {
    error_sender: Sender<WatchEvent>,
    // Sends the index of the updated exercise.
    update_sender: SyncSender<usize>,
    // Used to report which exercise was modified.
    exercise_names: &'static [&'static [u8]],
}

impl NotifyEventHandler {
    pub fn build(
        watch_event_sender: Sender<WatchEvent>,
        exercise_names: &'static [&'static [u8]],
    ) -> Result<Self> {
        let (update_sender, update_receiver) = sync_channel(0);
        let error_sender = watch_event_sender.clone();

        // Debouncer
        thread::Builder::new()
            .spawn(move || {
                let mut exercise_updated = vec![false; exercise_names.len()];

                loop {
                    match update_receiver.recv_timeout(DEBOUNCE_DURATION) {
                        Ok(exercise_ind) => exercise_updated[exercise_ind] = true,
                        Err(RecvTimeoutError::Timeout) => {
                            for (exercise_ind, updated) in exercise_updated.iter_mut().enumerate() {
                                if *updated {
                                    if watch_event_sender
                                        .send(WatchEvent::FileChange { exercise_ind })
                                        .is_err()
                                    {
                                        break;
                                    }

                                    *updated = false;
                                }
                            }
                        }
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
            })
            .context("Failed to spawn a thread to debounce file changes")?;

        Ok(Self {
            error_sender,
            update_sender,
            exercise_names,
        })
    }
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
                // After dropping the receiver, the watcher guard should also be dropped.
                let _ = self.error_sender.send(WatchEvent::NotifyErr(e));
                return;
            }
        };

        match input_event.kind {
            EventKind::Any => (),
            EventKind::Modify(modify_kind) => match modify_kind {
                ModifyKind::Any | ModifyKind::Data(_) => (),
                ModifyKind::Name(rename_mode) => match rename_mode {
                    RenameMode::Any | RenameMode::To => (),
                    RenameMode::From | RenameMode::Both | RenameMode::Other => return,
                },
                ModifyKind::Metadata(metadata_kind) => match metadata_kind {
                    MetadataKind::Any | MetadataKind::WriteTime => (),
                    MetadataKind::AccessTime
                    | MetadataKind::Permissions
                    | MetadataKind::Ownership
                    | MetadataKind::Extended
                    | MetadataKind::Other => return,
                },
                ModifyKind::Other => return,
            },
            EventKind::Access(access_kind) => match access_kind {
                AccessKind::Any => (),
                AccessKind::Close(access_mode) => match access_mode {
                    AccessMode::Any | AccessMode::Write => (),
                    AccessMode::Execute | AccessMode::Read | AccessMode::Other => return,
                },
                AccessKind::Read | AccessKind::Open(_) | AccessKind::Other => return,
            },
            EventKind::Create(_) | EventKind::Remove(_) | EventKind::Other => return,
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
            .try_for_each(|exercise_ind| self.update_sender.send(exercise_ind));
    }
}
