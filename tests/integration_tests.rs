use assert_cmd::prelude::*;
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
        .arg("v")
        .current_dir("tests/fixture/success")
        .assert()
        .success();
}

#[test]
fn verify_all_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compSuccess.rs"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compFailure.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testSuccess.rs"])
        .current_dir("tests/fixture/success/")
        .assert()
        .success();
}

#[test]
fn run_single_test_failure() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testFailure.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_not_passed() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testNotPassed.rs"])
        .current_dir("tests/fixture/failure/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_filename() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("r")
        .current_dir("tests/fixture/")
        .assert()
        .code(1);
}

#[test]
fn run_single_test_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compNoExercise.rs"])
        .current_dir("tests/fixture/failure")
        .assert()
        .code(1);
}
