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
        .failure();
}

#[test]
fn verify_all_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("v")
        .current_dir("tests/fixture/")
        .assert()
        .success();
}

#[test]
fn run_single_compile_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compSuccess.rs"])
        .current_dir("tests/fixture/")
        .assert()
        .success();
}

#[test]
fn run_single_test_success() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "testSuccess.rs"])
        .current_dir("tests/fixture/")
        .assert()
        .success();
}

#[test]
fn run_single_test_no_filename() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .arg("r")
        .current_dir("tests/fixture/")
        .assert()
        .failure();
}

#[test]
fn run_single_test_no_exercise() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["r", "compNoExercise.rs"])
        .current_dir("tests/fixture/")
        .assert()
        .failure();
}
