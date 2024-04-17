use anyhow::{bail, Context, Result};
use std::{
    cmp::Ordering,
    fs::{self, read_dir},
    path::PathBuf,
};

use crate::{
    info_file::{ExerciseInfo, InfoFile},
    CURRENT_FORMAT_VERSION, DEVELOPING_OFFICIAL_RUSTLINGS,
};

fn forbidden_char(input: &str) -> Option<char> {
    input.chars().find(|c| *c != '_' && !c.is_alphanumeric())
}

fn check_info_file_exercises(info_file: &InfoFile) -> Result<hashbrown::HashSet<PathBuf>> {
    let mut names = hashbrown::HashSet::with_capacity(info_file.exercises.len());
    let mut paths = hashbrown::HashSet::with_capacity(info_file.exercises.len());

    for exercise_info in &info_file.exercises {
        if exercise_info.name.is_empty() {
            bail!("Found an empty exercise name in `info.toml`");
        }
        if let Some(c) = forbidden_char(&exercise_info.name) {
            bail!(
                "Char `{c}` in the exercise name `{}` is not allowed",
                exercise_info.name,
            );
        }

        if let Some(dir) = &exercise_info.dir {
            if dir.is_empty() {
                bail!("Found an empty dir name in `info.toml`");
            }
            if let Some(c) = forbidden_char(dir) {
                bail!("Char `{c}` in the exercise dir `{dir}` is not allowed");
            }
        }

        if exercise_info.hint.is_empty() {
            bail!("The exercise `{}` has an empty hint. Please provide a hint or at least tell the user why a hint isn't needed for this exercise", exercise_info.name);
        }

        if !names.insert(exercise_info.name.as_str()) {
            bail!(
                "The exercise name {} is duplicated. Exercise names must all be unique",
                exercise_info.name,
            );
        }

        paths.insert(PathBuf::from(exercise_info.path()));
    }

    Ok(paths)
}

fn check_exercise_dir_files(
    info_file: &InfoFile,
    info_file_paths: hashbrown::HashSet<PathBuf>,
) -> Result<hashbrown::HashSet<String>> {
    let mut names = hashbrown::HashSet::with_capacity(info_file.exercises.len());

    for entry in read_dir("exercises").context("Failed to open the `exercises` directory")? {
        let entry = entry.context("Failed to read the `exercises` directory")?;

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !info_file_paths.contains(&path) {
                bail!("`{}` is expected to be an exercise file corresponding to some exercise in `info.toml`", path.display());
            }

            let file_name = file_name.to_string_lossy();
            names.insert(file_name[..file_name.len() - 3].to_string());
            continue;
        }

        let dir_path = entry.path();
        for entry in read_dir(&dir_path)
            .with_context(|| format!("Failed to open the directory {}", dir_path.display()))?
        {
            let entry = entry
                .with_context(|| format!("Failed to read the directory {}", dir_path.display()))?;
            let path = entry.path();

            if !entry.file_type().unwrap().is_file() {
                bail!("Found {} but expected only files", path.display());
            }

            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !info_file_paths.contains(&path) {
                bail!("`{}` is expected to be an exercise file corresponding to some exercise in `info.toml`", path.display());
            }

            // The file name must be valid Unicode with the `.rs` extension
            // because it is part of the info file paths.
            let file_name = file_name.to_string_lossy();
            let file_name_without_rs_extension = file_name[..file_name.len() - 3].to_string();
            names.insert(file_name_without_rs_extension);
        }
    }

    Ok(names)
}

fn check_info_file(info_file: &InfoFile) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!("`format_version` < {CURRENT_FORMAT_VERSION} (supported version)\nPlease migrate to the latest format version"),
        Ordering::Greater => bail!("`format_version` > {CURRENT_FORMAT_VERSION} (supported version)\nTry updating the Rustlings program"),
        Ordering::Equal => (),
    }

    let info_file_paths = check_info_file_exercises(info_file)?;
    let names_in_exercises_dir = check_exercise_dir_files(info_file, info_file_paths)?;

    // Now, we know that every file has an exercise in `info.toml`.
    // But we need to check that every exercise in `info.toml` has a file.
    if names_in_exercises_dir.len() != info_file.exercises.len() {
        for exercise_info in &info_file.exercises {
            if !names_in_exercises_dir.contains(&exercise_info.name) {
                bail!("The file `{}` is missing", exercise_info.path());
            }
        }
    }

    Ok(())
}

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
    check_info_file(&info_file)?;

    if DEVELOPING_OFFICIAL_RUSTLINGS {
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
