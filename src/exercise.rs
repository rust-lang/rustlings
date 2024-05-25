use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process::{self, exit, Command, Stdio};
use std::{array, env, mem};
use winnow::ascii::{space0, Caseless};
use winnow::combinator::opt;
use winnow::Parser;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_EDITION_ARGS: &[&str] = &["--edition", "2021"];
const RUSTC_NO_DEBUG_ARGS: &[&str] = &["-C", "strip=debuginfo"];
const CONTEXT: usize = 2;
const CLIPPY_CARGO_TOML_PATH: &str = "./exercises/22_clippy/Cargo.toml";

// Checks if the line contains the "I AM NOT DONE" comment.
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

// Get a temporary file name that is hopefully unique
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{thread_id}", process::id())
}

// The mode of the exercise.
#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    // Indicates that the exercise should be compiled as a binary
    Compile,
    // Indicates that the exercise should be compiled as a test harness
    Test,
    // Indicates that the exercise should be linted with clippy
    Clippy,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// A representation of a rustlings exercise.
// This is deserialized from the accompanying info.toml file
#[derive(Deserialize, Debug)]
pub struct Exercise {
    // Name of the exercise
    pub name: String,
    // The path to the file containing the exercise's source code
    pub path: PathBuf,
    // The mode of the exercise (Test, Compile, or Clippy)
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
}

// An enum to track of the state of an Exercise.
// An Exercise can be either Done or Pending
#[derive(PartialEq, Eq, Debug)]
pub enum State {
    // The state of the exercise once it's been completed
    Done,
    // The state of the exercise while it's not completed yet
    Pending(Vec<ContextLine>),
}

// The context information of a pending exercise
#[derive(PartialEq, Eq, Debug)]
pub struct ContextLine {
    // The source code that is still pending completion
    pub line: String,
    // The line number of the source code still pending completion
    pub number: usize,
    // Whether or not this is important
    pub important: bool,
}

// The result of compiling an exercise
pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

impl<'a> CompiledExercise<'a> {
    // Run the compiled exercise
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

// A representation of an already executed binary
#[derive(Debug)]
pub struct ExerciseOutput {
    // The textual contents of the standard output of the binary
    pub stdout: String,
    // The textual contents of the standard error of the binary
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
                .args([self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .args(RUSTC_NO_DEBUG_ARGS)
                .output(),
            Mode::Test => Command::new("rustc")
                .args(["--test", self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .args(RUSTC_NO_DEBUG_ARGS)
                .output(),
            Mode::Clippy => {
                let cargo_toml = format!(
                    r#"[package]
name = "{}"
version = "0.0.1"
edition = "2021"
[[bin]]
name = "{}"
path = "{}.rs""#,
                    self.name, self.name, self.name
                );
                let cargo_toml_error_msg = if env::var("NO_EMOJI").is_ok() {
                    "Failed to write Clippy Cargo.toml file."
                } else {
                    "Failed to write 📎 Clippy 📎 Cargo.toml file."
                };
                fs::write(CLIPPY_CARGO_TOML_PATH, cargo_toml).expect(cargo_toml_error_msg);
                // To support the ability to run the clippy exercises, build
                // an executable, in addition to running clippy. With a
                // compilation failure, this would silently fail. But we expect
                // clippy to reflect the same failure while compiling later.
                Command::new("rustc")
                    .args([self.path.to_str().unwrap(), "-o", &temp_file()])
                    .args(RUSTC_COLOR_ARGS)
                    .args(RUSTC_EDITION_ARGS)
                    .args(RUSTC_NO_DEBUG_ARGS)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("Failed to compile!");
                // Due to an issue with Clippy, a cargo clean is required to catch all lints.
                // See https://github.com/rust-lang/rust-clippy/issues/2604
                // This is already fixed on Clippy's master branch. See this issue to track merging into Cargo:
                // https://github.com/rust-lang/rust-clippy/issues/3837
                Command::new("cargo")
                    .args(["clean", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("Failed to run 'cargo clean'");
                Command::new("cargo")
                    .args(["clippy", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .args(["--", "-D", "warnings", "-D", "clippy::float_cmp"])
                    .output()
            }
        }
        .expect("Failed to run 'compile' command.");

        if cmd.status.success() {
            Ok(CompiledExercise {
                exercise: self,
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
        let arg = match self.mode {
            Mode::Test => "--show-output",
            _ => "",
        };
        let cmd = Command::new(temp_file())
            .arg(arg)
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
        let source_file = File::open(&self.path).unwrap_or_else(|e| {
            println!(
                "Failed to open the exercise file {}: {e}",
                self.path.display(),
            );
            exit(1);
        });
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
            let n = read_line(&mut line).unwrap_or_else(|e| {
                println!(
                    "Failed to read the exercise file {}: {e}",
                    self.path.display(),
                );
                exit(1);
            });

            // Reached the end of the file and didn't find the comment.
            if n == 0 {
                return State::Done;
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

                return State::Pending(context);
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
    pub fn looks_done(&self) -> bool {
        self.state() == State::Done
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

#[inline]
fn clean() {
    let _ignored = remove_file(temp_file());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_clean() {
        File::create(temp_file()).unwrap();
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
    #[cfg(target_os = "windows")]
    fn test_no_pdb_file() {
        [Mode::Compile, Mode::Test] // Clippy doesn't like to test
            .iter()
            .for_each(|mode| {
                let exercise = Exercise {
                    name: String::from("example"),
                    // We want a file that does actually compile
                    path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
                    mode: *mode,
                    hint: String::from(""),
                };
                let _ = exercise.compile().unwrap();
                assert!(!Path::new(&format!("{}.pdb", temp_file())).exists());
            });
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

    #[test]
    fn test_exercise_with_output() {
        let exercise = Exercise {
            name: "exercise_with_output".into(),
            path: PathBuf::from("tests/fixture/success/testSuccess.rs"),
            mode: Mode::Test,
            hint: String::new(),
        };
        let out = exercise.compile().unwrap().run().unwrap();
        assert!(out.stdout.contains("THIS TEST TOO SHALL PASS"));
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
