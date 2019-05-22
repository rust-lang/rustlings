use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::remove_file;
use std::path::PathBuf;
use std::process::{self, Command, Output};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];

fn temp_file() -> String {
    format!("./temp_{}", process::id())
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
pub struct Exercise {
    pub path: PathBuf,
    pub mode: Mode,
}

impl Exercise {
    pub fn compile(&self) -> Output {
        match self.mode {
            Mode::Compile => Command::new("rustc")
                .args(&[self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .output(),
            Mode::Test => Command::new("rustc")
                .args(&["--test", self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .output(),
        }
        .expect("Failed to run 'compile' command.")
    }

    pub fn run(&self) -> Output {
        Command::new(&temp_file())
            .output()
            .expect("Failed to run 'run' command")
    }

    pub fn clean(&self) {
        let _ignored = remove_file(&temp_file());
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_clean() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            path: PathBuf::from("example.rs"),
            mode: Mode::Test,
        };
        exercise.clean();
        assert!(!Path::new(&temp_file()).exists());
    }
}
