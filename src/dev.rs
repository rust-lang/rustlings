use anyhow::{bail, Context, Result};
use clap::Subcommand;

use crate::DEVELOPING_OFFICIAL_RUSTLINGS;

mod check;
mod init;
mod update;

#[derive(Subcommand)]
pub enum DevCommands {
    Init,
    Check,
    Update,
}

impl DevCommands {
    pub fn run(self) -> Result<()> {
        match self {
            DevCommands::Init => {
                if DEVELOPING_OFFICIAL_RUSTLINGS {
                    bail!("Disabled while developing the official Rustlings");
                }

                init::init().context(INIT_ERR)
            }
            DevCommands::Check => check::check(),
            DevCommands::Update => update::update(),
        }
    }
}

const INIT_ERR: &str = "Initialization failed.
After resolving the issue, delete the `rustlings` directory (if it was created) and try again";
