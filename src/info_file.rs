use anyhow::{bail, Context, Error, Result};
use serde::Deserialize;
use std::{fs, io::ErrorKind};

use crate::DEBUG_PROFILE;

// The mode of the exercise.
#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    // The exercise should be compiled as a binary
    Run,
    // The exercise should be compiled as a test harness
    Test,
    // The exercise should be linted with clippy
    Clippy,
}

// Deserialized from the `info.toml` file.
#[derive(Deserialize)]
pub struct ExerciseInfo {
    // Name of the exercise
    pub name: String,
    // The exercise's directory inside the `exercises` directory
    pub dir: Option<String>,
    // The mode of the exercise
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
}

impl ExerciseInfo {
    pub fn path(&self) -> String {
        if let Some(dir) = &self.dir {
            format!("exercises/{dir}/{}.rs", self.name)
        } else {
            format!("exercises/{}.rs", self.name)
        }
    }
}

#[derive(Deserialize)]
pub struct InfoFile {
    pub format_version: u8,
    pub welcome_message: Option<String>,
    pub final_message: Option<String>,
    pub exercises: Vec<ExerciseInfo>,
}

impl InfoFile {
    fn from_embedded() -> Result<Self> {
        toml_edit::de::from_str(include_str!("../info.toml"))
            .context("Failed to parse the embedded `info.toml` file")
    }

    pub fn parse() -> Result<Self> {
        if DEBUG_PROFILE {
            return Self::from_embedded();
        }

        // Read a local `info.toml` if it exists.
        let slf = match fs::read_to_string("info.toml") {
            Ok(file_content) => toml_edit::de::from_str::<Self>(&file_content)
                .context("Failed to parse the `info.toml` file")?,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Self::from_embedded();
                }

                return Err(Error::from(e).context("Failed to read the `info.toml` file"));
            }
        };

        if slf.exercises.is_empty() {
            bail!("{NO_EXERCISES_ERR}");
        }

        Ok(slf)
    }
}

const NO_EXERCISES_ERR: &str = "There are no exercises yet!
If you are developing third-party exercises, add at least one exercise before testing.";
