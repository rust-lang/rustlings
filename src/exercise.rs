use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::{self},
    path::PathBuf,
    process::{Command, Output},
};

use crate::embedded::{WriteStrategy, EMBEDDED_FILES};

// The mode of the exercise.
#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    // The exercise should be compiled as a binary
    Compile,
    // The exercise should be compiled as a test harness
    Test,
    // The exercise should be linted with clippy
    Clippy,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoFile {
    // TODO
    pub welcome_message: Option<String>,
    pub final_message: Option<String>,
    pub exercises: Vec<Exercise>,
}

impl InfoFile {
    pub fn parse() -> Result<Self> {
        // Read a local `info.toml` if it exists.
        // Mainly to let the tests work for now.
        let slf: Self = if let Ok(file_content) = fs::read_to_string("info.toml") {
            toml_edit::de::from_str(&file_content)
        } else {
            toml_edit::de::from_str(include_str!("../info.toml"))
        }
        .context("Failed to parse `info.toml`")?;

        if slf.exercises.is_empty() {
            panic!("{NO_EXERCISES_ERR}");
        }

        Ok(slf)
    }
}

// Deserialized from the `info.toml` file.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exercise {
    // Name of the exercise
    pub name: String,
    // The path to the file containing the exercise's source code
    pub path: PathBuf,
    // The mode of the exercise
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
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
            .arg(&self.name)
            .args(args)
            .output()
            .context("Failed to run Cargo")
    }

    pub fn run(&self) -> Result<Output> {
        match self.mode {
            Mode::Compile => self.cargo_cmd("run", &[]),
            Mode::Test => self.cargo_cmd("test", &["--", "--nocapture", "--format", "pretty"]),
            Mode::Clippy => self.cargo_cmd(
                "clippy",
                &["--", "-D", "warnings", "-D", "clippy::float_cmp"],
            ),
        }
    }

    pub fn reset(&self) -> Result<()> {
        EMBEDDED_FILES
            .write_exercise_to_disk(&self.path, WriteStrategy::Overwrite)
            .with_context(|| format!("Failed to reset the exercise {self}"))
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.path.display(), f)
    }
}

const NO_EXERCISES_ERR: &str = "There are no exercises yet!
If you are developing third-party exercises, add at least one exercise before testing.";
