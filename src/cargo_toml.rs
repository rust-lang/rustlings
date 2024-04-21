use anyhow::{Context, Result};

use crate::info_file::ExerciseInfo;

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

pub fn updated_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
) -> Result<Vec<u8>> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let mut updated_cargo_toml = Vec::with_capacity(1 << 13);
    updated_cargo_toml.extend_from_slice(current_cargo_toml[..bins_start_ind].as_bytes());
    append_bins(
        &mut updated_cargo_toml,
        exercise_infos,
        exercise_path_prefix,
    );
    updated_cargo_toml.extend_from_slice(current_cargo_toml[bins_end_ind..].as_bytes());

    Ok(updated_cargo_toml)
}
