use regex::Regex;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{self, Command};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;
const CLIPPY_CARGO_TOML_PATH: &str = "./exercises/clippy/Cargo.toml";

fn temp_file() -> String {
    format!("./temp_{}", process::id())
}

#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
    Clippy,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
    pub hint: String,
}

#[derive(PartialEq, Debug)]
pub enum State {
    Done,
    Pending(Vec<ContextLine>),
}

#[derive(PartialEq, Debug)]
pub struct ContextLine {
    pub line: String,
    pub number: usize,
    pub important: bool,
}

pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

impl<'a> CompiledExercise<'a> {
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

#[derive(Debug)]
pub struct ExerciseOutput {
    pub stdout: String,
    pub stderr: String,
}

struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

impl Exercise {
    pub fn compile(&self) -> Result<CompiledExercise, ExerciseOutput> {
        let cmd = match self.mode {
            Mode::Compile => Command::new("rustc")
                .args(&[self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .output(),
            Mode::Test => Command::new("rustc")
                .args(&["--test", self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .output(),
            Mode::Clippy => {
                let cargo_toml = format!(
                    r#"[package]
name = "{}"
version = "0.0.1"
edition = "2018"
[[bin]]
name = "{}"
path = "{}.rs""#,
                    self.name, self.name, self.name
                );
                fs::write(CLIPPY_CARGO_TOML_PATH, cargo_toml)
                    .expect("Failed to write 📎 Clippy 📎 Cargo.toml file.");
                // To support the ability to run the clipy exercises, build
                // an executable, in addition to running clippy. With a
                // compilation failure, this would silently fail. But we expect
                // clippy to reflect the same failure while compiling later.
                Command::new("rustc")
                    .args(&[self.path.to_str().unwrap(), "-o", &temp_file()])
                    .args(RUSTC_COLOR_ARGS)
                    .output()
                    .expect("Failed to compile!");
                // Due to an issue with Clippy, a cargo clean is required to catch all lints.
                // See https://github.com/rust-lang/rust-clippy/issues/2604
                // This is already fixed on master branch. See this issue to track merging into Cargo:
                // https://github.com/rust-lang/rust-clippy/issues/3837
                Command::new("cargo")
                    .args(&["clean", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .output()
                    .expect("Failed to run 'cargo clean'");
                Command::new("cargo")
                    .args(&["clippy", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .args(&["--", "-D", "warnings"])
                    .output()
            }
        }
        .expect("Failed to run 'compile' command.");

        if cmd.status.success() {
            Ok(CompiledExercise {
                exercise: &self,
                _handle: FileHandle,
            })
        } else {
            clean();
            Err(ExerciseOutput {
                stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
            })
        }
    }

    fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let cmd = Command::new(&temp_file())
            .output()
            .expect("Failed to run 'run' command");

        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        };

        if cmd.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }

    pub fn state(&self) -> State {
        let mut source_file =
            File::open(&self.path).expect("We were unable to open the exercise file!");

        let source = {
            let mut s = String::new();
            source_file
                .read_to_string(&mut s)
                .expect("We were unable to read the exercise file!");
            s
        };

        let re = Regex::new(I_AM_DONE_REGEX).unwrap();

        if !re.is_match(&source) {
            return State::Done;
        }

        let matched_line_index = source
            .lines()
            .enumerate()
            .filter_map(|(i, line)| if re.is_match(line) { Some(i) } else { None })
            .next()
            .expect("This should not happen at all");

        let min_line = ((matched_line_index as i32) - (CONTEXT as i32)).max(0) as usize;
        let max_line = matched_line_index + CONTEXT;

        let context = source
            .lines()
            .enumerate()
            .filter(|&(i, _)| i >= min_line && i <= max_line)
            .map(|(i, line)| ContextLine {
                line: line.to_string(),
                number: i + 1,
                important: i == matched_line_index,
            })
            .collect();

        State::Pending(context)
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

fn clean() {
    let _ignored = remove_file(&temp_file());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_clean() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
            mode: Mode::Compile,
            hint: String::from(""),
        };
        let compiled = exercise.compile().unwrap();
        drop(compiled);
        assert!(!Path::new(&temp_file()).exists());
    }

    #[test]
    fn test_pending_state() {
        let exercise = Exercise {
            name: "pending_exercise".into(),
            path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
            mode: Mode::Compile,
            hint: String::new(),
        };

        let state = exercise.state();
        let expected = vec![
            ContextLine {
                line: "// fake_exercise".to_string(),
                number: 1,
                important: false,
            },
            ContextLine {
                line: "".to_string(),
                number: 2,
                important: false,
            },
            ContextLine {
                line: "// I AM NOT DONE".to_string(),
                number: 3,
                important: true,
            },
            ContextLine {
                line: "".to_string(),
                number: 4,
                important: false,
            },
            ContextLine {
                line: "fn main() {".to_string(),
                number: 5,
                important: false,
            },
        ];

        assert_eq!(state, State::Pending(expected));
    }

    #[test]
    fn test_finished_exercise() {
        let exercise = Exercise {
            name: "finished_exercise".into(),
            path: PathBuf::from("tests/fixture/state/finished_exercise.rs"),
            mode: Mode::Compile,
            hint: String::new(),
        };

        assert_eq!(exercise.state(), State::Done);
    }
}
