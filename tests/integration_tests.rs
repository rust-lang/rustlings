use std::{
    env::{self, consts::EXE_SUFFIX},
    process::{Command, Stdio},
    str::from_utf8,
};

enum Output<'a> {
    FullStdout(&'a str),
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

    fn assert(&self, success: bool) {
        let rustlings_bin = {
            let mut path = env::current_exe().unwrap();
            // Pop test binary name
            path.pop();
            // Pop `/deps`
            path.pop();

            path.push("rustlings");
            let mut path = path.into_os_string();
            path.push(EXE_SUFFIX);
            path
        };

        let mut cmd = Command::new(rustlings_bin);

        if let Some(current_dir) = self.current_dir {
            cmd.current_dir(current_dir);
        }

        cmd.args(self.args).stdin(Stdio::null());

        let status = match self.output {
            None => cmd
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap(),
            Some(FullStdout(stdout)) => {
                let output = cmd.stderr(Stdio::null()).output().unwrap();
                assert_eq!(from_utf8(&output.stdout).unwrap(), stdout);
                output.status
            }
            Some(PartialStdout(stdout)) => {
                let output = cmd.stderr(Stdio::null()).output().unwrap();
                assert!(from_utf8(&output.stdout).unwrap().contains(stdout));
                output.status
            }
            Some(PartialStderr(stderr)) => {
                let output = cmd.stdout(Stdio::null()).output().unwrap();
                assert!(from_utf8(&output.stderr).unwrap().contains(stderr));
                output.status
            }
        };

        assert_eq!(status.success(), success, "{cmd:?}");
    }

    #[inline]
    fn success(&self) {
        self.assert(true);
    }

    #[inline]
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
        .output(FullStdout("The answer to everything: 42\n"))
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
