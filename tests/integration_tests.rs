use std::{
    env::{self, consts::EXE_SUFFIX},
    process::{Command, Stdio},
    str::from_utf8,
};

#[derive(Default)]
struct Cmd<'a> {
    current_dir: Option<&'a str>,
    args: &'a [&'a str],
    stdout: Option<&'a str>,
    full_stdout: bool,
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
    fn stdout(&mut self, stdout: &'a str) -> &mut Self {
        self.stdout = Some(stdout);
        self
    }

    #[inline]
    fn full_stdout(&mut self) -> &mut Self {
        self.full_stdout = true;
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

        cmd.args(self.args)
            .stdin(Stdio::null())
            .stderr(Stdio::null());

        let status = if let Some(expected_stdout) = self.stdout {
            let output = cmd.output().unwrap();
            let stdout = from_utf8(&output.stdout).unwrap();

            if self.full_stdout {
                assert_eq!(stdout, expected_stdout);
            } else {
                assert!(stdout.contains(expected_stdout));
            }

            output.status
        } else {
            cmd.stdout(Stdio::null()).status().unwrap()
        };

        assert_eq!(status.success(), success);
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
fn fails_when_in_wrong_dir() {
    Cmd::default().current_dir("tests").fail();
}

#[test]
fn run_single_compile_success() {
    Cmd::default()
        .current_dir("tests/fixture/success")
        .args(&["run", "compSuccess"])
        .success();
}

#[test]
fn run_single_compile_failure() {
    Cmd::default()
        .current_dir("tests/fixture/failure")
        .args(&["run", "compFailure"])
        .fail();
}

#[test]
fn run_single_test_success() {
    Cmd::default()
        .current_dir("tests/fixture/success")
        .args(&["run", "testSuccess"])
        .success();
}

#[test]
fn run_single_test_failure() {
    Cmd::default()
        .current_dir("tests/fixture/failure")
        .args(&["run", "testFailure"])
        .fail();
}

#[test]
fn run_single_test_not_passed() {
    Cmd::default()
        .current_dir("tests/fixture/failure")
        .args(&["run", "testNotPassed.rs"])
        .fail();
}

#[test]
fn run_single_test_no_exercise() {
    Cmd::default()
        .current_dir("tests/fixture/failure")
        .args(&["run", "compNoExercise.rs"])
        .fail();
}

#[test]
fn reset_single_exercise() {
    Cmd::default().args(&["reset", "intro1"]).success();
}

#[test]
fn reset_no_exercise() {
    Cmd::default().args(&["reset"]).fail();
}

#[test]
fn get_hint_for_single_test() {
    Cmd::default()
        .current_dir("tests/fixture/failure")
        .args(&["hint", "testFailure"])
        .stdout("Hello!\n")
        .full_stdout()
        .success();
}

#[test]
fn run_compile_exercise_does_not_prompt() {
    Cmd::default()
        .current_dir("tests/fixture/state")
        .args(&["run", "pending_exercise"])
        .success();
}

#[test]
fn run_test_exercise_does_not_prompt() {
    Cmd::default()
        .current_dir("tests/fixture/state")
        .args(&["run", "pending_test_exercise"])
        .success();
}

#[test]
fn run_single_test_success_with_output() {
    Cmd::default()
        .current_dir("tests/fixture/success")
        .args(&["run", "testSuccess"])
        .stdout("\nTHIS TEST TOO SHALL PASS\n")
        .success();
}
