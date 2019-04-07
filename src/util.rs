use std::fs::remove_file;
use std::process::{Command, Output};

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];

pub fn compile_test_cmd(filename: &str) -> Output {
    Command::new("rustc")
        .args(&["--test", filename, "-o", "temp"])
        .args(RUSTC_COLOR_ARGS)
        .output()
        .expect("failed to compile exercise")
}

pub fn compile_cmd(filename: &str) -> Output {
    Command::new("rustc")
        .args(&[filename, "-o", "temp"])
        .args(RUSTC_COLOR_ARGS)
        .output()
        .expect("failed to compile exercise")
}

pub fn run_cmd() -> Output {
    Command::new("./temp")
        .output()
        .expect("failed to run exercise")
}

pub fn clean() {
    let _ignored = remove_file("temp");
}

#[test]
fn test_clean() {
    std::fs::File::create("temp").unwrap();
    clean();
    assert!(!std::path::Path::new("temp").exists());
}
