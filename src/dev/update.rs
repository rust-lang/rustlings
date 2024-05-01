use anyhow::{Context, Result};
use std::fs;

use crate::{
    cargo_toml::updated_cargo_toml,
    info_file::{ExerciseInfo, InfoFile},
    DEBUG_PROFILE,
};

// Update the `Cargo.toml` file.
fn update_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
    cargo_toml_path: &str,
) -> Result<()> {
    let updated_cargo_toml =
        updated_cargo_toml(exercise_infos, current_cargo_toml, exercise_path_prefix)?;

    fs::write(cargo_toml_path, updated_cargo_toml)
        .context("Failed to write the `Cargo.toml` file")?;

    Ok(())
}

pub fn update() -> Result<()> {
    let info_file = InfoFile::parse()?;

    // A hack to make `cargo run -- dev update` work when developing Rustlings.
    if DEBUG_PROFILE {
        update_cargo_toml(
            &info_file.exercises,
            include_str!("../../dev-Cargo.toml"),
            b"../",
            "dev/Cargo.toml",
        )
        .context("Failed to update the file `dev/Cargo.toml`")?;

        println!("Updated `dev/Cargo.toml`");
    } else {
        let current_cargo_toml =
            fs::read_to_string("Cargo.toml").context("Failed to read the file `Cargo.toml`")?;
        update_cargo_toml(&info_file.exercises, &current_cargo_toml, b"", "Cargo.toml")
            .context("Failed to update the file `Cargo.toml`")?;

        println!("Updated `Cargo.toml`");
    }

    Ok(())
}
