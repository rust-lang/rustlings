use anyhow::{Context, Result};
use clap::Subcommand;

use crate::info_file::InfoFile;

mod check;
mod init;

#[derive(Subcommand)]
pub enum DevCommands {
    Init,
    Check,
}

impl DevCommands {
    pub fn run(self, info_file: InfoFile) -> Result<()> {
        match self {
            DevCommands::Init => init::init().context(INIT_ERR),
            DevCommands::Check => check::check(info_file),
        }
    }
}

const INIT_ERR: &str = "Initialization failed.
After resolving the issue, delete the `rustlings` directory (if it was created) and try again";
