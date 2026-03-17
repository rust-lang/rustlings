use std::{
    process::{Command, Stdio},
    str::from_utf8,
};

enum Output<'a> {
    FullStdout(&'a [u8]),
    PartialStdout(&'a str),
    PartialStderr(&'a str),
}

use Output::*;

#[derive(Default)]
struct Cmd<'a> {
    current_dir: Option<&'a str>,
    args: &'a [&'a str],
    output: Option<Output<'a>>,
}

impl<'a> Cmd<'a> {
    #[inline]
    fn current_dir(&mut self, current_dir: &'a str) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
    }

    #[inline]
    fn args(&mut self, args: &'a [&'a str]) -> &mut Self {
        self.args = args;
        self
    }

    #[inline]
    fn output(&mut self, output: Output<'a>) -> &mut Self {
        self.output = Some(output);
        self
    }

    #[track_caller]
    fn assert(&self, success: bool) {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_rustlings"));

        if let Some(current_dir) = self.current_dir {
            cmd.current_dir(current_dir);
        }

        cmd.args(self.args).stdin(Stdio::null());

        let output = cmd.output().unwrap();
        match self.output {
            None => (),
            Some(FullStdout(stdout)) => {
                assert_eq!(output.stdout, stdout);
            }
            Some(PartialStdout(stdout)) => {
                assert!(from_utf8(&output.stdout).unwrap().contains(stdout));
            }
            Some(PartialStderr(stderr)) => {
                assert!(from_utf8(&output.stderr).unwrap().contains(stderr));
            }
        };

        if output.status.success() != success {
            panic!(
                "{cmd:?}\n\nstdout:\n{}\n\nstderr:\n{}",
                from_utf8(&output.stdout).unwrap(),
                from_utf8(&output.stderr).unwrap(),
            );
        }
    }

    #[inline]
    #[track_caller]
    fn success(&self) {
        self.assert(true);
    }

    #[inline]
    #[track_caller]
    fn fail(&self) {
        self.assert(false);
    }
}

#[test]
fn run_compilation_success() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["run", "compilation_success"])
        .success();
}

#[test]
fn run_compilation_failure() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["run", "compilation_failure"])
        .fail();
}

#[test]
fn run_test_success() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["run", "test_success"])
        .output(PartialStdout("\nOutput from `main` function\n"))
        .success();
}

#[test]
fn run_test_failure() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["run", "test_failure"])
        .fail();
}

#[test]
fn run_exercise_not_in_info() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["run", "not_in_info"])
        .fail();
}

#[test]
fn reset_without_exercise_name() {
    Cmd::default().args(&["reset"]).fail();
}

#[test]
fn hint() {
    Cmd::default()
        .current_dir("tests/test_exercises")
        .args(&["hint", "test_failure"])
        .output(FullStdout(b"The answer to everything: 42\n"))
        .success();
}

#[test]
fn init() {
    let test_dir = tempfile::TempDir::new().unwrap();
    let test_dir = test_dir.path().to_str().unwrap();

    Cmd::default().current_dir(test_dir).fail();

    Cmd::default()
        .current_dir(test_dir)
        .args(&["init"])
        .success();

    // Running `init` after a successful initialization.
    Cmd::default()
        .current_dir(test_dir)
        .args(&["init"])
        .output(PartialStderr("`cd rustlings`"))
        .fail();

    let initialized_dir = format!("{test_dir}/rustlings");

    // Running `init` in the initialized directory.
    Cmd::default()
        .current_dir(&initialized_dir)
        .args(&["init"])
        .output(PartialStderr("already initialized"))
        .fail();
}
