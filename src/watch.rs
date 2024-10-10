use anyhow::{Error, Result};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    io::{self, Write},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        mpsc::channel,
    },
    time::Duration,
};

use crate::{
    app_state::{AppState, ExercisesProgress},
    list,
};

use self::{notify_event::NotifyEventHandler, state::WatchState, terminal_event::InputEvent};

mod notify_event;
mod state;
mod terminal_event;

static EXERCISE_RUNNING: AtomicBool = AtomicBool::new(false);

// Private unit type to force using the constructor function.
#[must_use = "When the guard is dropped, the input is unpaused"]
pub struct InputPauseGuard(());

impl InputPauseGuard {
    #[inline]
    pub fn scoped_pause() -> Self {
        EXERCISE_RUNNING.store(true, Relaxed);
        Self(())
    }
}

impl Drop for InputPauseGuard {
    #[inline]
    fn drop(&mut self) {
        EXERCISE_RUNNING.store(false, Relaxed);
    }
}

enum WatchEvent {
    Input(InputEvent),
    FileChange { exercise_ind: usize },
    TerminalResize { width: u16 },
    NotifyErr(notify::Error),
    TerminalEventErr(io::Error),
}

/// Returned by the watch mode to indicate what to do afterwards.
#[must_use]
enum WatchExit {
    /// Exit the program.
    Shutdown,
    /// Enter the list mode and restart the watch mode afterwards.
    List,
}

fn run_watch(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<WatchExit> {
    let (watch_event_sender, watch_event_receiver) = channel();

    let mut manual_run = false;
    // Prevent dropping the guard until the end of the function.
    // Otherwise, the file watcher exits.
    let _watcher_guard = if let Some(exercise_names) = notify_exercise_names {
        let notify_event_handler =
            NotifyEventHandler::build(watch_event_sender.clone(), exercise_names)?;

        let mut watcher = RecommendedWatcher::new(
            notify_event_handler,
            Config::default().with_poll_interval(Duration::from_secs(1)),
        )
        .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        watcher
            .watch(Path::new("exercises"), RecursiveMode::Recursive)
            .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        Some(watcher)
    } else {
        manual_run = true;
        None
    };

    let mut watch_state = WatchState::build(app_state, watch_event_sender, manual_run)?;
    let mut stdout = io::stdout().lock();

    watch_state.run_current_exercise(&mut stdout)?;

    while let Ok(event) = watch_event_receiver.recv() {
        match event {
            WatchEvent::Input(InputEvent::Next) => match watch_state.next_exercise(&mut stdout)? {
                ExercisesProgress::AllDone => break,
                ExercisesProgress::NewPending => watch_state.run_current_exercise(&mut stdout)?,
                ExercisesProgress::CurrentPending => (),
            },
            WatchEvent::Input(InputEvent::Run) => watch_state.run_current_exercise(&mut stdout)?,
            WatchEvent::Input(InputEvent::Hint) => watch_state.show_hint(&mut stdout)?,
            WatchEvent::Input(InputEvent::List) => return Ok(WatchExit::List),
            WatchEvent::Input(InputEvent::CheckAll) => match watch_state
                .check_all_exercises(&mut stdout)?
            {
                ExercisesProgress::AllDone => break,
                ExercisesProgress::NewPending => watch_state.run_current_exercise(&mut stdout)?,
                ExercisesProgress::CurrentPending => watch_state.render(&mut stdout)?,
            },
            WatchEvent::Input(InputEvent::Reset) => watch_state.reset_exercise(&mut stdout)?,
            WatchEvent::Input(InputEvent::Quit) => {
                stdout.write_all(QUIT_MSG)?;
                break;
            }
            WatchEvent::FileChange { exercise_ind } => {
                watch_state.handle_file_change(exercise_ind, &mut stdout)?;
            }
            WatchEvent::TerminalResize { width } => {
                watch_state.update_term_width(width, &mut stdout)?;
            }
            WatchEvent::NotifyErr(e) => return Err(Error::from(e).context(NOTIFY_ERR)),
            WatchEvent::TerminalEventErr(e) => {
                return Err(Error::from(e).context("Terminal event listener failed"));
            }
        }
    }

    Ok(WatchExit::Shutdown)
}

fn watch_list_loop(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<()> {
    loop {
        match run_watch(app_state, notify_exercise_names)? {
            WatchExit::Shutdown => break Ok(()),
            // It is much easier to exit the watch mode, launch the list mode and then restart
            // the watch mode instead of trying to pause the watch threads and correct the
            // watch state.
            WatchExit::List => list::list(app_state)?,
        }
    }
}

/// `notify_exercise_names` as None activates the manual run mode.
pub fn watch(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<()> {
    #[cfg(not(windows))]
    {
        let stdin_fd = rustix::stdio::stdin();
        let mut termios = rustix::termios::tcgetattr(stdin_fd)?;
        let original_local_modes = termios.local_modes;
        // Disable stdin line buffering and hide input.
        termios.local_modes -=
            rustix::termios::LocalModes::ICANON | rustix::termios::LocalModes::ECHO;
        rustix::termios::tcsetattr(stdin_fd, rustix::termios::OptionalActions::Now, &termios)?;

        let res = watch_list_loop(app_state, notify_exercise_names);

        termios.local_modes = original_local_modes;
        rustix::termios::tcsetattr(stdin_fd, rustix::termios::OptionalActions::Now, &termios)?;

        res
    }

    #[cfg(windows)]
    watch_list_loop(app_state, notify_exercise_names)
}

const QUIT_MSG: &[u8] = b"

We hope you're enjoying learning Rust!
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again in this directory.
";

const NOTIFY_ERR: &str = "
The automatic detection of exercise file changes failed :(
Please try running `rustlings` again.

If you keep getting this error, run `rustlings --manual-run` to deactivate the file watcher.
You need to manually trigger running the current exercise using `r` then.
";
