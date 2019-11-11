use regex::Regex;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::{remove_file, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::{self, Command, Output};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;

fn temp_file() -> String {
    format!("./temp_{}", process::id())
}

#[derive(Deserialize, Copy, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_clean() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("example.rs"),
            mode: Mode::Test,
            hint: String::from(""),
        };
        exercise.clean();
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
