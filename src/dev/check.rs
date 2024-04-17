use anyhow::{bail, Context, Result};
use std::fs;

use crate::{
    info_file::{ExerciseInfo, InfoFile},
    DEVELOPING_OFFIFICAL_RUSTLINGS,
};

pub fn bins_start_end_ind(cargo_toml: &str) -> Result<(usize, usize)> {
    let start_ind = cargo_toml
        .find("bin = [")
        .context("Failed to find the start of the `bin` list (`bin = [`)")?
        + 7;
    let end_ind = start_ind
        + cargo_toml
            .get(start_ind..)
            .and_then(|slice| slice.as_bytes().iter().position(|c| *c == b']'))
            .context("Failed to find the end of the `bin` list (`]`)")?;

    Ok((start_ind, end_ind))
}

pub fn append_bins(
    buf: &mut Vec<u8>,
    exercise_infos: &[ExerciseInfo],
    exercise_path_prefix: &[u8],
) {
    buf.push(b'\n');
    for exercise_info in exercise_infos {
        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b"\", path = \"");
        buf.extend_from_slice(exercise_path_prefix);
        buf.extend_from_slice(b"exercises/");
        if let Some(dir) = &exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");
    }
}

fn check_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let old_bins = &current_cargo_toml.as_bytes()[bins_start_ind..bins_end_ind];
    let mut new_bins = Vec::with_capacity(1 << 13);
    append_bins(&mut new_bins, exercise_infos, exercise_path_prefix);

    if old_bins != new_bins {
        bail!("`Cargo.toml` is outdated. Run `rustlings dev update` to update it");
    }

    Ok(())
}

pub fn check() -> Result<()> {
    let info_file = InfoFile::parse()?;

    // TODO: Add checks

    if DEVELOPING_OFFIFICAL_RUSTLINGS {
        check_cargo_toml(
            &info_file.exercises,
            include_str!("../../dev/Cargo.toml"),
            b"../",
        )
        .context("The file `dev/Cargo.toml` is outdated. Please run `cargo run -- dev update` to update it")?;
    } else {
        let current_cargo_toml =
            fs::read_to_string("Cargo.toml").context("Failed to read the file `Cargo.toml`")?;
        check_cargo_toml(&info_file.exercises, &current_cargo_toml, b"").context(
            "The file `Cargo.toml` is outdated. Please run `rustlings dev update` to update it",
        )?;
    }

    println!("\nEverything looks fine!");

    Ok(())
}
