use anyhow::{bail, Context, Result};
use app_state::StateFileStatus;
use clap::{Parser, Subcommand};
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, BufRead, Write},
    path::Path,
    process::exit,
};

use self::{app_state::AppState, dev::DevCommands, info_file::InfoFile, watch::WatchExit};

mod app_state;
mod cargo_toml;
mod dev;
mod embedded;
mod exercise;
mod info_file;
mod init;
mod list;
mod progress_bar;
mod run;
mod watch;

const CURRENT_FORMAT_VERSION: u8 = 1;
const DEBUG_PROFILE: bool = {
    #[allow(unused_assignments, unused_mut)]
    let mut debug_profile = false;

    #[cfg(debug_assertions)]
    {
        debug_profile = true;
    }

    debug_profile
};

/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
    /// Manually run the current exercise using `r` or `run` in the watch mode.
    /// Only use this if Rustlings fails to detect exercise file changes.
    #[arg(long)]
    manual_run: bool,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Initialize Rustlings
    Init,
    /// Run a single exercise. Runs the next pending exercise if the exercise name is not specified.
    Run {
        /// The name of the exercise
        name: Option<String>,
    },
    /// Reset a single exercise
    Reset {
        /// The name of the exercise
        name: String,
    },
    /// Show a hint. Shows the hint of the next pending exercise if the exercise name is not specified.
    Hint {
        /// The name of the exercise
        name: Option<String>,
    },
    #[command(subcommand)]
    Dev(DevCommands),
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !DEBUG_PROFILE && Path::new("dev/rustlings-repo.txt").exists() {
        bail!("{OLD_METHOD_ERR}");
    }

    which::which("cargo").context(CARGO_NOT_FOUND_ERR)?;

    match args.command {
        Some(Subcommands::Init) => {
            if DEBUG_PROFILE {
                bail!("Disabled in the debug build");
            }

            {
                let mut stdout = io::stdout().lock();
                stdout.write_all(b"This command will create the directory `rustlings/` which will contain the exercises.\nPress ENTER to continue ")?;
                stdout.flush()?;
                io::stdin().lock().read_until(b'\n', &mut Vec::new())?;
                stdout.write_all(b"\n")?;
            }

            return init::init().context("Initialization failed");
        }
        Some(Subcommands::Dev(dev_command)) => return dev_command.run(),
        _ => (),
    }

    if !Path::new("exercises").is_dir() {
        println!("{PRE_INIT_MSG}");
        exit(1);
    }

    let info_file = InfoFile::parse()?;

    if info_file.format_version > CURRENT_FORMAT_VERSION {
        bail!(FORMAT_VERSION_HIGHER_ERR);
    }

    let (mut app_state, state_file_status) = AppState::new(
        info_file.exercises,
        info_file.final_message.unwrap_or_default(),
    );

    if let Some(welcome_message) = info_file.welcome_message {
        match state_file_status {
            StateFileStatus::NotRead => {
                let mut stdout = io::stdout().lock();
                stdout.execute(Clear(ClearType::All))?;

                let welcome_message = welcome_message.trim();
                write!(stdout, "{welcome_message}\n\nPress ENTER to continue ")?;
                stdout.flush()?;

                io::stdin().lock().read_until(b'\n', &mut Vec::new())?;

                stdout.execute(Clear(ClearType::All))?;
            }
            StateFileStatus::Read => (),
        }
    }

    match args.command {
        None => {
            let notify_exercise_paths: Option<&'static [&'static str]> = if args.manual_run {
                None
            } else {
                // For the the notify event handler thread.
                // Leaking is not a problem because the slice lives until the end of the program.
                Some(
                    app_state
                        .exercises()
                        .iter()
                        .map(|exercise| exercise.path)
                        .collect::<Vec<_>>()
                        .leak(),
                )
            };

            loop {
                match watch::watch(&mut app_state, notify_exercise_paths)? {
                    WatchExit::Shutdown => break,
                    // It is much easier to exit the watch mode, launch the list mode and then restart
                    // the watch mode instead of trying to pause the watch threads and correct the
                    // watch state.
                    WatchExit::List => list::list(&mut app_state)?,
                }
            }
        }
        Some(Subcommands::Run { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }
            run::run(&mut app_state)?;
        }
        Some(Subcommands::Reset { name }) => {
            app_state.set_current_exercise_by_name(&name)?;
            let exercise_path = app_state.reset_current_exercise()?;
            println!("The exercise {exercise_path} has been reset");
        }
        Some(Subcommands::Hint { name }) => {
            if let Some(name) = name {
                app_state.set_current_exercise_by_name(&name)?;
            }
            println!("{}", app_state.current_exercise().hint);
        }
        // Handled in an earlier match.
        Some(Subcommands::Init | Subcommands::Dev(_)) => (),
    }

    Ok(())
}

const OLD_METHOD_ERR: &str = "You are trying to run Rustlings using the old method before v6.
The new method doesn't include cloning the Rustlings' repository.
Please follow the instructions in the README:
https://github.com/rust-lang/rustlings#getting-started";

const CARGO_NOT_FOUND_ERR: &str = "Failed to find `cargo`.
Did you already install Rust?
Try running `cargo --version` to diagnose the problem.";

const FORMAT_VERSION_HIGHER_ERR: &str =
    "The format version specified in the `info.toml` file is higher than the last one supported.
It is possible that you have an outdated version of Rustlings.
Try to install the latest Rustlings version first.";

const PRE_INIT_MSG: &str = r"
       Welcome to...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/

The `exercises` directory wasn't found in the current directory.
If you are just starting with Rustlings, run the command `rustlings init` to initialize it.";
