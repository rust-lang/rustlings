// Makes sure that `dev/Cargo.toml` is synced with `info.toml`.
// When this test fails, you just need to run `cargo run -p gen-dev-cargo-toml`.

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ExerciseInfo {
    name: String,
    dir: Option<String>,
}

#[derive(Deserialize)]
struct InfoFile {
    exercises: Vec<ExerciseInfo>,
}

#[test]
fn dev_cargo_bins() {
    let cargo_toml = fs::read_to_string("dev/Cargo.toml").unwrap();

    let exercise_infos =
        toml_edit::de::from_str::<InfoFile>(&fs::read_to_string("info.toml").unwrap())
            .unwrap()
            .exercises;

    let mut start_ind = 0;
    for exercise_info in exercise_infos {
        let name_start = start_ind + cargo_toml[start_ind..].find('"').unwrap() + 1;
        let name_end = name_start + cargo_toml[name_start..].find('"').unwrap();
        assert_eq!(exercise_info.name, &cargo_toml[name_start..name_end]);

        let path_start = name_end + cargo_toml[name_end + 1..].find('"').unwrap() + 2;
        let path_end = path_start + cargo_toml[path_start..].find('"').unwrap();
        let expected_path = if let Some(dir) = exercise_info.dir {
            format!("../exercises/{dir}/{}.rs", exercise_info.name)
        } else {
            format!("../exercises/{}.rs", exercise_info.name)
        };
        assert_eq!(expected_path, &cargo_toml[path_start..path_end]);

        start_ind = path_end + 1;
    }
}
