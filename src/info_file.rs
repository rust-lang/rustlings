use anyhow::{bail, Context, Error, Result};
use serde::Deserialize;
use std::{fs, io::ErrorKind};

use crate::embedded::EMBEDDED_FILES;

// Deserialized from the `info.toml` file.
#[derive(Deserialize)]
pub struct ExerciseInfo {
    // Name of the exercise
    pub name: String,
    // The exercise's directory inside the `exercises` directory
    pub dir: Option<String>,
    #[serde(default = "default_true")]
    pub test: bool,
    #[serde(default)]
    pub strict_clippy: bool,
    // The hint text associated with the exercise
    pub hint: String,
}
#[inline]
const fn default_true() -> bool {
    true
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
    pub fn parse() -> Result<Self> {
        // Read a local `info.toml` if it exists.
        let slf = match fs::read_to_string("info.toml") {
            Ok(file_content) => toml_edit::de::from_str::<Self>(&file_content)
                .context("Failed to parse the `info.toml` file")?,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return toml_edit::de::from_str(EMBEDDED_FILES.info_file)
                        .context("Failed to parse the embedded `info.toml` file");
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
