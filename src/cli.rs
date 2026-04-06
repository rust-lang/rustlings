use clap::{Parser, Subcommand};

use crate::dev::DevCommand;

/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// Open the current exercise by running `EDIT_CMD EXERCISE_PATH`.
    /// The command is not allowed to block (e.g. `vim`).
    /// It should communicate with an editor in a different process.
    /// `EDIT_CMD` can contain arguments like `--edit-cmd "PROGRAM -x --arg1"`.
    /// The current exercise's path is added by Rustlings as the last argument.
    /// `--edit-cmd` is ignored in VS Code.
    ///
    /// Example: `--edit-cmd "code"` (default behavior if running in a VS Code terminal)
    #[arg(long)]
    pub edit_cmd: Option<String>,
    /// Manually run the current exercise using `r` in the watch mode.
    /// Only use this if Rustlings fails to detect exercise file changes
    #[arg(long)]
    pub manual_run: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize the official Rustlings exercises
    Init,
    /// Run a single exercise.
    /// Runs the next pending exercise if the exercise name is not specified
    Run {
        /// The name of the exercise
        name: Option<String>,
    },
    /// Check all the exercises, marking them as done or pending accordingly
    CheckAll,
    /// Reset a single exercise
    Reset {
        /// The name of the exercise
        name: String,
    },
    /// Show a hint.
    /// Shows the hint of the next pending exercise if the exercise name is not specified
    Hint {
        /// The name of the exercise
        name: Option<String>,
    },
    /// Commands for developing (community) Rustlings exercises
    #[command(subcommand)]
    Dev(DevCommand),
}
