use assert_cmd::prelude::*;
use glob::glob;
use predicates::boolean::PredicateBooleanExt;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn runs_without_arguments() {
    let mut cmd = Command::cargo_bin("rustlings").unwrap();
    cmd.assert().success();
}

#[test]
fn fails_when_in_wrong_dir() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .current_dir("tests/")
        .assert()
        .code(1);
}

#[test]
fn verify_all_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("verify")
        .current_dir("tests/fixture/success")
        .assert()
        .success();
}

#[test]
fn verify_fails_if_some_fails() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("verify")
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "compSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "compFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_test_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "testFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_not_passed() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "testNotPassed.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_filename() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("run")
        .current_dir("tests/fixture/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "compNoExercise.rs"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn get_hint_for_single_test() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["hint", "testFailure"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(0)
        .stdout("Hello!\n");
}

#[test]
fn all_exercises_require_confirmation() {
    for exercise in glob("exercises/**/*.rs").unwrap() {
        let path = exercise.unwrap();
        let source = {
            let mut file = File::open(&path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();
            s
        };
        source
            .matches("// I AM NOT DONE")
            .next()
            .unwrap_or_else(|| {
                panic!(
                    "There should be an `I AM NOT DONE` annotation in {:?}",
                    path
                )
            });
    }
}

#[test]
fn run_compile_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "pending_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_test_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "pending_test_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("I AM NOT DONE").not());
}

#[test]
fn run_single_test_success_with_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["--nocapture", "run", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS"));
}

#[test]
fn run_single_test_success_without_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["run", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PAS").not());
}

#[test]
fn run_rustlings_list() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["list"])
        .current_dir("tests/fixture/success")
        .assert()
        .success();
}

#[test]
fn run_rustlings_list_no_pending() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["list"])
        .current_dir("tests/fixture/success")
        .assert()
        .success()
        .stdout(predicates::str::contains("Pending").not());
}

#[test]
fn run_rustlings_list_both_done_and_pending() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["list"])
        .current_dir("tests/fixture/state")
        .assert()
        .success()
        .stdout(predicates::str::contains("Done").and(predicates::str::contains("Pending")));
}

#[test]
fn run_rustlings_list_without_pending() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["list", "--solved"])
        .current_dir("tests/fixture/state")
        .assert()
        .success()
        .stdout(predicates::str::contains("Pending").not());
}

#[test]
fn run_rustlings_list_without_done() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["list", "--unsolved"])
        .current_dir("tests/fixture/state")
        .assert()
        .success()
        .stdout(predicates::str::contains("Done").not());
}
