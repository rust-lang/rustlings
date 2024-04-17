use std::fs;

use anyhow::{Context, Result};

use crate::{
    info_file::{ExerciseInfo, InfoFile},
    DEVELOPING_OFFIFICAL_RUSTLINGS,
};

use super::check::{append_bins, bins_start_end_ind};

fn update_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let mut new_cargo_toml = Vec::with_capacity(1 << 13);
    new_cargo_toml.extend_from_slice(current_cargo_toml[..bins_start_ind].as_bytes());
    append_bins(&mut new_cargo_toml, exercise_infos, exercise_path_prefix);
    new_cargo_toml.extend_from_slice(current_cargo_toml[bins_end_ind..].as_bytes());

    fs::write(cargo_toml_path, new_cargo_toml).context("Failed to write the `Cargo.toml` file")?;

    Ok(())
}

pub fn update() -> Result<()> {
    let info_file = InfoFile::parse()?;

    if DEVELOPING_OFFIFICAL_RUSTLINGS {
        update_cargo_toml(
            &info_file.exercises,
            include_str!("../../dev/Cargo.toml"),
            "dev/Cargo.toml",
            b"../",
        )
        .context("Failed to update the file `dev/Cargo.toml`")?;

        println!("Updated `dev/Cargo.toml`");
    } else {
        let current_cargo_toml =
            fs::read_to_string("Cargo.toml").context("Failed to read the file `Cargo.toml`")?;
        update_cargo_toml(&info_file.exercises, &current_cargo_toml, "Cargo.toml", b"")
            .context("Failed to update the file `Cargo.toml`")?;

        println!("Updated `Cargo.toml`");
    }

    Ok(())
}
