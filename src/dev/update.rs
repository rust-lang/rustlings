use anyhow::{Context, Result};
use std::fs;

use crate::{
    cargo_toml::updated_cargo_toml,
    info_file::{ExerciseInfo, InfoFile},
};

// Update the `Cargo.toml` file.
fn update_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let current_cargo_toml = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("Failed to read the file `{cargo_toml_path}`"))?;

    let updated_cargo_toml =
        updated_cargo_toml(exercise_infos, &current_cargo_toml, exercise_path_prefix)?;

    fs::write(cargo_toml_path, updated_cargo_toml)
        .context("Failed to write the `Cargo.toml` file")?;

    Ok(())
}

pub fn update() -> Result<()> {
    let info_file = InfoFile::parse()?;

    if cfg!(debug_assertions) {
        // A hack to make `cargo run -- dev update` work when developing Rustlings.
        update_cargo_toml(&info_file.exercises, "dev/Cargo.toml", b"../")
            .context("Failed to update the file `dev/Cargo.toml`")?;

        println!("Updated `dev/Cargo.toml`");
    } else {
        update_cargo_toml(&info_file.exercises, "Cargo.toml", &[])
            .context("Failed to update the file `Cargo.toml`")?;

        println!("Updated `Cargo.toml`");
    }

    Ok(())
}
