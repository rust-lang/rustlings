use anyhow::Result;
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
            DevCommands::Init => init::init(),
            DevCommands::Check => check::check(),
        }
    }
}
