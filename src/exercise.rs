use anyhow::{Context, Result};
use crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    process::{Command, Output},
};

use crate::{info_file::Mode, terminal_link::TerminalFileLink, DEBUG_PROFILE};

pub struct Exercise {
    pub dir: Option<&'static str>,
    // Exercise's unique name
    pub name: &'static str,
    // Exercise's path
    pub path: &'static str,
    // The mode of the exercise
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    fn cargo_cmd(&self, command: &str, args: &[&str]) -> Result<Output> {
        let mut cmd = Command::new("cargo");
        cmd.arg(command);

        // A hack to make `cargo run` work when developing Rustlings.
        if DEBUG_PROFILE && Path::new("tests").exists() {
            cmd.arg("--manifest-path").arg("dev/Cargo.toml");
        }

        cmd.arg("--color")
            .arg("always")
            .arg("-q")
            .arg("--bin")
            .arg(self.name)
            .args(args)
            .output()
            .context("Failed to run Cargo")
    }

    pub fn run(&self) -> Result<Output> {
        match self.mode {
            Mode::Run => self.cargo_cmd("run", &[]),
            Mode::Test => self.cargo_cmd(
                "test",
                &[
                    "--",
                    "--color",
                    "always",
                    "--nocapture",
                    "--format",
                    "pretty",
                ],
            ),
            Mode::Clippy => self.cargo_cmd(
                "clippy",
                &["--", "-D", "warnings", "-D", "clippy::float_cmp"],
            ),
        }
    }

    pub fn terminal_link(&self) -> StyledContent<TerminalFileLink<'_>> {
        style(TerminalFileLink(self.path)).underlined().blue()
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}
