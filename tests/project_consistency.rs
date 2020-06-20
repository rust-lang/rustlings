use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn all_exercises_require_confirmation() {
    for path in all_exercises() {
        let source = fs::read_to_string(&path).unwrap();
        source.matches("// I AM NOT DONE").next().expect(&format!(
            "There should be an `I AM NOT DONE` annotation in {:?}",
            path
        ));
    }
}

#[test]
fn cargo_toml_is_up_to_date() {
    let exercises_manifest = fs::read_to_string("exercises/Cargo.toml").unwrap();
    let generated_manifest = tooling::generate(&Path::new("exercises"));
    assert_eq!(
        generated_manifest, exercises_manifest,
        "exercises/Cargo.toml is not up to date, run `cargo generate-manifest`"
    );
}

fn all_exercises() -> impl Iterator<Item = PathBuf> {
    glob("exercises/**/*.rs")
        .unwrap()
        .map(|result| result.expect("Unable to traverse exercises folder"))
}
