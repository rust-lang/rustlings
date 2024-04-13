use anyhow::{Context, Result};
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    process::{Command, Output},
};

use crate::{
    embedded::{WriteStrategy, EMBEDDED_FILES},
    info_file::Mode,
};

pub struct Exercise {
    // Exercise's unique name
    pub name: &'static str,
    // Exercise's path
    pub path: &'static Path,
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
        // Use `dev/Cargo.toml` when in the directory of the repository.
        #[cfg(debug_assertions)]
        if std::path::Path::new("tests").exists() {
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
            Mode::Test => self.cargo_cmd("test", &["--", "--nocapture", "--format", "pretty"]),
            Mode::Clippy => self.cargo_cmd(
                "clippy",
                &["--", "-D", "warnings", "-D", "clippy::float_cmp"],
            ),
        }
    }

    pub fn reset(&self) -> Result<()> {
        EMBEDDED_FILES
            .write_exercise_to_disk(self.path, WriteStrategy::Overwrite)
            .with_context(|| format!("Failed to reset the exercise {self}"))
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.path.display(), f)
    }
}
