use anyhow::{Context, Error, Result};
use notify_debouncer_mini::{
    new_debouncer,
    notify::{self, RecursiveMode},
};
use std::{
    io::{self, Write},
    path::Path,
    sync::mpsc::channel,
    thread,
    time::Duration,
};

use crate::{
    app_state::{AppState, ExercisesProgress},
    list,
};

use self::{
    notify_event::NotifyEventHandler,
    state::WatchState,
    terminal_event::{terminal_event_handler, InputEvent},
};

mod notify_event;
mod state;
mod terminal_event;

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
    let (tx, rx) = channel();

    let mut manual_run = false;
    // Prevent dropping the guard until the end of the function.
    // Otherwise, the file watcher exits.
    let _debouncer_guard = if let Some(exercise_names) = notify_exercise_names {
        let mut debouncer = new_debouncer(
            Duration::from_millis(200),
            NotifyEventHandler {
                tx: tx.clone(),
                exercise_names,
            },
        )
        .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;
        debouncer
            .watcher()
            .watch(Path::new("exercises"), RecursiveMode::Recursive)
            .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        Some(debouncer)
    } else {
        manual_run = true;
        None
    };

    let mut watch_state = WatchState::build(app_state, manual_run)?;

    let mut stdout = io::stdout().lock();
    watch_state.run_current_exercise(&mut stdout)?;

    thread::Builder::new()
        .spawn(move || terminal_event_handler(tx, manual_run))
        .context("Failed to spawn a thread to handle terminal events")?;

    while let Ok(event) = rx.recv() {
        match event {
            WatchEvent::Input(InputEvent::Next) => match watch_state.next_exercise(&mut stdout)? {
                ExercisesProgress::AllDone => break,
                ExercisesProgress::NewPending => watch_state.run_current_exercise(&mut stdout)?,
                ExercisesProgress::CurrentPending => (),
            },
            WatchEvent::Input(InputEvent::Hint) => watch_state.show_hint(&mut stdout)?,
            WatchEvent::Input(InputEvent::List) => return Ok(WatchExit::List),
            WatchEvent::Input(InputEvent::Quit) => {
                stdout.write_all(QUIT_MSG)?;
                break;
            }
            WatchEvent::Input(InputEvent::Run) => watch_state.run_current_exercise(&mut stdout)?,
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
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again.
";

const NOTIFY_ERR: &str = "
The automatic detection of exercise file changes failed :(
Please try running `rustlings` again.

If you keep getting this error, run `rustlings --manual-run` to deactivate the file watcher.
You need to manually trigger running the current exercise using `r` then.
";
