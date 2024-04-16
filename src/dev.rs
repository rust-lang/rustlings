use anyhow::{Context, Result};
use clap::Subcommand;

mod check;
mod init;

#[derive(Subcommand)]
pub enum DevCommands {
    Init,
    Check,
}

impl DevCommands {
    pub fn run(self) -> Result<()> {
        match self {
            DevCommands::Init => init::init().context(INIT_ERR),
            DevCommands::Check => check::check(),
        }
    }
}

const INIT_ERR: &str = "Initialization failed.
After resolving the issue, delete the `rustlings` directory (if it was created) and try again";
