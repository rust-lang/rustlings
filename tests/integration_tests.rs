use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn fails_when_in_wrong_dir() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .current_dir("tests/")
        .assert()
        .code(1);
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "compSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "compFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_test_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "testFailure"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_not_passed() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "testNotPassed.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "compNoExercise.rs"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn reset_single_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["reset", "intro1"])
        .assert()
        .code(0);
}

#[test]
fn reset_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("reset")
        .assert()
        .code(2)
        .stderr(predicates::str::contains(
            "required arguments were not provided",
        ));
}

#[test]
fn get_hint_for_single_test() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["hint", "testFailure"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(0)
        .stdout("Hello!\n");
}

#[test]
fn run_compile_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "pending_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0);
}

#[test]
fn run_test_exercise_does_not_prompt() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "pending_test_exercise"])
        .current_dir("tests/fixture/state")
        .assert()
        .code(0);
}

#[test]
fn run_single_test_success_with_output() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(["run", "testSuccess"])
        .current_dir("tests/fixture/success/")
        .assert()
        .code(0)
        .stdout(predicates::str::contains("THIS TEST TOO SHALL PASS"));
}
