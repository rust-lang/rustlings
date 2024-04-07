use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    array,
    fmt::{self, Debug, Display, Formatter},
    fs::{self, File},
    io::{self, BufRead, BufReader},
    mem,
    path::PathBuf,
    process::{Command, Output},
};
use winnow::{
    ascii::{space0, Caseless},
    combinator::opt,
    Parser,
};

use crate::embedded::{WriteStrategy, EMBEDDED_FILES};

// The number of context lines above and below a highlighted line.
const CONTEXT: usize = 2;

// Check if the line contains the "I AM NOT DONE" comment.
fn contains_not_done_comment(input: &str) -> bool {
    (
        space0::<_, ()>,
        "//",
        opt('/'),
        space0,
        Caseless("I AM NOT DONE"),
    )
        .parse_next(&mut &*input)
        .is_ok()
}

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
pub struct InfoFile {
    pub exercises: Vec<Exercise>,
}

impl InfoFile {
    pub fn parse() -> Result<Self> {
        // Read a local `info.toml` if it exists.
        // Mainly to let the tests work for now.
        if let Ok(file_content) = fs::read_to_string("info.toml") {
            toml_edit::de::from_str(&file_content)
        } else {
            toml_edit::de::from_str(include_str!("../info.toml"))
        }
        .context("Failed to parse `info.toml`")
    }
}

// Deserialized from the `info.toml` file.
#[derive(Deserialize)]
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

// The state of an Exercise.
#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Done,
    Pending(Vec<ContextLine>),
}

// The context information of a pending exercise.
#[derive(PartialEq, Eq, Debug)]
pub struct ContextLine {
    // The source code line
    pub line: String,
    // The line number
    pub number: usize,
    // Whether this is important and should be highlighted
    pub important: bool,
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

    pub fn state(&self) -> Result<State> {
        let source_file = File::open(&self.path)
            .with_context(|| format!("Failed to open the exercise file {}", self.path.display()))?;
        let mut source_reader = BufReader::new(source_file);

        // Read the next line into `buf` without the newline at the end.
        let mut read_line = |buf: &mut String| -> io::Result<_> {
            let n = source_reader.read_line(buf)?;
            if buf.ends_with('\n') {
                buf.pop();
                if buf.ends_with('\r') {
                    buf.pop();
                }
            }
            Ok(n)
        };

        let mut current_line_number: usize = 1;
        // Keep the last `CONTEXT` lines while iterating over the file lines.
        let mut prev_lines: [_; CONTEXT] = array::from_fn(|_| String::with_capacity(256));
        let mut line = String::with_capacity(256);

        loop {
            let n = read_line(&mut line).with_context(|| {
                format!("Failed to read the exercise file {}", self.path.display())
            })?;

            // Reached the end of the file and didn't find the comment.
            if n == 0 {
                return Ok(State::Done);
            }

            if contains_not_done_comment(&line) {
                let mut context = Vec::with_capacity(2 * CONTEXT + 1);
                // Previous lines.
                for (ind, prev_line) in prev_lines
                    .into_iter()
                    .take(current_line_number - 1)
                    .enumerate()
                    .rev()
                {
                    context.push(ContextLine {
                        line: prev_line,
                        number: current_line_number - 1 - ind,
                        important: false,
                    });
                }

                // Current line.
                context.push(ContextLine {
                    line,
                    number: current_line_number,
                    important: true,
                });

                // Next lines.
                for ind in 0..CONTEXT {
                    let mut next_line = String::with_capacity(256);
                    let Ok(n) = read_line(&mut next_line) else {
                        // If an error occurs, just ignore the next lines.
                        break;
                    };

                    // Reached the end of the file.
                    if n == 0 {
                        break;
                    }

                    context.push(ContextLine {
                        line: next_line,
                        number: current_line_number + 1 + ind,
                        important: false,
                    });
                }

                return Ok(State::Pending(context));
            }

            current_line_number += 1;
            // Add the current line as a previous line and shift the older lines by one.
            for prev_line in &mut prev_lines {
                mem::swap(&mut line, prev_line);
            }
            // The current line now contains the oldest previous line.
            // Recycle it for reading the next line.
            line.clear();
        }
    }

    // Check that the exercise looks to be solved using self.state()
    // This is not the best way to check since
    // the user can just remove the "I AM NOT DONE" string from the file
    // without actually having solved anything.
    // The only other way to truly check this would to compile and run
    // the exercise; which would be both costly and counterintuitive
    pub fn looks_done(&self) -> Result<bool> {
        self.state().map(|state| state == State::Done)
    }

    pub fn reset(&self) -> Result<()> {
        EMBEDDED_FILES
            .write_exercise_to_disk(&self.path, WriteStrategy::Overwrite)
            .with_context(|| format!("Failed to reset the exercise {self}"))
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pending_state() {
        let exercise = Exercise {
            name: "pending_exercise".into(),
            path: PathBuf::from("tests/fixture/state/exercises/pending_exercise.rs"),
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

        assert_eq!(state.unwrap(), State::Pending(expected));
    }

    #[test]
    fn test_finished_exercise() {
        let exercise = Exercise {
            name: "finished_exercise".into(),
            path: PathBuf::from("tests/fixture/state/exercises/finished_exercise.rs"),
            mode: Mode::Compile,
            hint: String::new(),
        };

        assert_eq!(exercise.state().unwrap(), State::Done);
    }

    #[test]
    fn test_not_done() {
        assert!(contains_not_done_comment("// I AM NOT DONE"));
        assert!(contains_not_done_comment("/// I AM NOT DONE"));
        assert!(contains_not_done_comment("//  I AM NOT DONE"));
        assert!(contains_not_done_comment("///  I AM NOT DONE"));
        assert!(contains_not_done_comment("// I AM NOT DONE "));
        assert!(contains_not_done_comment("// I AM NOT DONE!"));
        assert!(contains_not_done_comment("// I am not done"));
        assert!(contains_not_done_comment("// i am NOT done"));

        assert!(!contains_not_done_comment("I AM NOT DONE"));
        assert!(!contains_not_done_comment("// NOT DONE"));
        assert!(!contains_not_done_comment("DONE"));
    }
}
