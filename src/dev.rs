use anyhow::{bail, Context, Result};
use clap::Subcommand;
use std::path::PathBuf;

use crate::DEBUG_PROFILE;

mod check;
mod new;
mod update;

#[derive(Subcommand)]
pub enum DevCommands {
    /// Create a new project for third-party Rustlings exercises
    New {
        /// The path to create the project in
        path: PathBuf,
        /// Don't try to initialize a Git repository in the project directory
        #[arg(long)]
        no_git: bool,
    },
    /// Run checks on the exercises
    Check {
        /// Require that every exercise has a solution
        #[arg(short, long)]
        require_solutions: bool,
    },
    /// Update the `Cargo.toml` file for the exercises
    Update,
}

impl DevCommands {
    pub fn run(self) -> Result<()> {
        match self {
            Self::New { path, no_git } => {
                if DEBUG_PROFILE {
                    bail!("Disabled in the debug build");
                }

                new::new(&path, no_git).context(INIT_ERR)
            }
            Self::Check { require_solutions } => check::check(require_solutions),
            Self::Update => update::update(),
        }
    }
}

const INIT_ERR: &str = "Initialization failed.
After resolving the issue, delete the `rustlings` directory (if it was created) and try again";
