use anyhow::{bail, Context, Result};
use crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    fs,
    path::Path,
    process::{Command, Output, Stdio},
};

use crate::{
    embedded::{WriteStrategy, EMBEDDED_FILES},
    info_file::Mode,
    DEVELOPING_OFFICIAL_RUSTLINGS,
};

pub struct TerminalFileLink<'a> {
    path: &'a str,
}

impl<'a> Display for TerminalFileLink<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(Some(canonical_path)) = fs::canonicalize(self.path)
            .as_deref()
            .map(|path| path.to_str())
        {
            write!(
                f,
                "\x1b]8;;file://{}\x1b\\{}\x1b]8;;\x1b\\",
                canonical_path, self.path,
            )
        } else {
            write!(f, "{}", self.path,)
        }
    }
}

pub struct Exercise {
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
        if DEVELOPING_OFFICIAL_RUSTLINGS && Path::new("tests").exists() {
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

    pub fn reset(&self) -> Result<()> {
        if Path::new("info.toml").exists() {
            let output = Command::new("git")
                .arg("stash")
                .arg("push")
                .arg("--")
                .arg(self.path)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .output()
                .with_context(|| format!("Failed to run `git stash push -- {}`", self.path))?;

            if !output.status.success() {
                bail!(
                    "`git stash push -- {}` didn't run successfully: {}",
                    self.path,
                    String::from_utf8_lossy(&output.stderr),
                );
            }
        } else {
            EMBEDDED_FILES
                .write_exercise_to_disk(self.path, WriteStrategy::Overwrite)
                .with_context(|| format!("Failed to reset the exercise {self}"))?;
        }

        Ok(())
    }

    pub fn terminal_link(&self) -> StyledContent<TerminalFileLink<'_>> {
        style(TerminalFileLink { path: self.path })
            .underlined()
            .blue()
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}
