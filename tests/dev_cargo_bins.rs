// Makes sure that `dev/Cargo.toml` is synced with `info.toml`.
// When this test fails, you just need to run `cargo run --bin gen-dev-cargo-toml`.

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Exercise {
    name: String,
    path: String,
}

#[derive(Deserialize)]
struct InfoToml {
    exercises: Vec<Exercise>,
}

#[test]
fn dev_cargo_bins() {
    let content = fs::read_to_string("exercises/Cargo.toml").unwrap();

    let exercises = toml_edit::de::from_str::<InfoToml>(&fs::read_to_string("info.toml").unwrap())
        .unwrap()
        .exercises;

    let mut start_ind = 0;
    for exercise in exercises {
        let name_start = start_ind + content[start_ind..].find('"').unwrap() + 1;
        let name_end = name_start + content[name_start..].find('"').unwrap();
        assert_eq!(exercise.name, &content[name_start..name_end]);

        // +3 to skip `../` at the begeinning of the path.
        let path_start = name_end + content[name_end + 1..].find('"').unwrap() + 5;
        let path_end = path_start + content[path_start..].find('"').unwrap();
        assert_eq!(exercise.path, &content[path_start..path_end]);

        start_ind = path_end + 1;
    }
}
