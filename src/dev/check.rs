use anyhow::{anyhow, bail, Context, Error, Result};
use std::{
    cmp::Ordering,
    fs::{self, read_dir, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    app_state::parse_target_dir,
    cargo_toml::{append_bins, bins_start_end_ind, BINS_BUFFER_CAPACITY},
    exercise::{RunnableExercise, OUTPUT_CAPACITY},
    info_file::{ExerciseInfo, InfoFile},
    CURRENT_FORMAT_VERSION, DEBUG_PROFILE,
};

// Find a char that isn't allowed in the exercise's `name` or `dir`.
fn forbidden_char(input: &str) -> Option<char> {
    input.chars().find(|c| !c.is_alphanumeric() && *c != '_')
}

// Check that the Cargo.toml file is up-to-date.
fn check_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let old_bins = &current_cargo_toml.as_bytes()[bins_start_ind..bins_end_ind];
    let mut new_bins = Vec::with_capacity(BINS_BUFFER_CAPACITY);
    append_bins(&mut new_bins, exercise_infos, exercise_path_prefix);

    if old_bins != new_bins {
        if DEBUG_PROFILE {
            bail!("The file `dev/Cargo.toml` is outdated. Please run `cargo run -- dev update` to update it");
        }

        bail!("The file `Cargo.toml` is outdated. Please run `rustlings dev update` to update it");
    }

    Ok(())
}

// Check the info of all exercises and return their paths in a set.
fn check_info_file_exercises(info_file: &InfoFile) -> Result<hashbrown::HashSet<PathBuf>> {
    let mut names = hashbrown::HashSet::with_capacity(info_file.exercises.len());
    let mut paths = hashbrown::HashSet::with_capacity(info_file.exercises.len());

    let mut file_buf = String::with_capacity(1 << 14);
    for exercise_info in &info_file.exercises {
        let name = exercise_info.name.as_str();
        if name.is_empty() {
            bail!("Found an empty exercise name in `info.toml`");
        }
        if let Some(c) = forbidden_char(name) {
            bail!("Char `{c}` in the exercise name `{name}` is not allowed");
        }

        if let Some(dir) = &exercise_info.dir {
            if dir.is_empty() {
                bail!("The exercise `{name}` has an empty dir name in `info.toml`");
            }
            if let Some(c) = forbidden_char(dir) {
                bail!("Char `{c}` in the exercise dir `{dir}` is not allowed");
            }
        }

        if exercise_info.hint.trim().is_empty() {
            bail!("The exercise `{name}` has an empty hint. Please provide a hint or at least tell the user why a hint isn't needed for this exercise");
        }

        if !names.insert(name) {
            bail!("The exercise name `{name}` is duplicated. Exercise names must all be unique");
        }

        let path = exercise_info.path();

        OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("Failed to open the file {path}"))?
            .read_to_string(&mut file_buf)
            .with_context(|| format!("Failed to read the file {path}"))?;

        if !file_buf.contains("fn main()") {
            bail!("The `main` function is missing in the file `{path}`.\nCreate at least an empty `main` function to avoid language server errors");
        }

        if !exercise_info.test && file_buf.contains("#[test]") {
            bail!("The file `{path}` contains tests annotated with `#[test]` but the exercise `{name}` has `test = false` in the `info.toml` file");
        }

        file_buf.clear();

        paths.insert(PathBuf::from(path));
    }

    Ok(paths)
}

// Check the `exercises` directory for unexpected files.
fn check_unexpected_files(info_file_paths: &hashbrown::HashSet<PathBuf>) -> Result<()> {
    fn unexpected_file(path: &Path) -> Error {
        anyhow!("Found the file `{}`. Only `README.md` and Rust files related to an exercise in `info.toml` are allowed in the `exercises` directory", path.display())
    }

    for entry in read_dir("exercises").context("Failed to open the `exercises` directory")? {
        let entry = entry.context("Failed to read the `exercises` directory")?;

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !info_file_paths.contains(&path) {
                return Err(unexpected_file(&path));
            }

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
                bail!("Found `{}` but expected only files. Only one level of exercise nesting is allowed", path.display());
            }

            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !info_file_paths.contains(&path) {
                return Err(unexpected_file(&path));
            }
        }
    }

    Ok(())
}

fn check_exercises(info_file: &InfoFile) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!("`format_version` < {CURRENT_FORMAT_VERSION} (supported version)\nPlease migrate to the latest format version"),
        Ordering::Greater => bail!("`format_version` > {CURRENT_FORMAT_VERSION} (supported version)\nTry updating the Rustlings program"),
        Ordering::Equal => (),
    }

    let info_file_paths = check_info_file_exercises(info_file)?;
    check_unexpected_files(&info_file_paths)?;

    Ok(())
}

fn check_solutions(info_file: &InfoFile) -> Result<()> {
    let target_dir = parse_target_dir()?;
    let mut output = Vec::with_capacity(OUTPUT_CAPACITY);

    for exercise_info in &info_file.exercises {
        let success = exercise_info.run_solution(&mut output, &target_dir)?;
        if !success {
            io::stderr().write_all(&output)?;

            bail!(
                "Failed to run the solution of the exercise {}",
                exercise_info.name,
            );
        }
    }

    Ok(())
}

pub fn check() -> Result<()> {
    let info_file = InfoFile::parse()?;

    // A hack to make `cargo run -- dev check` work when developing Rustlings.
    if DEBUG_PROFILE {
        check_cargo_toml(
            &info_file.exercises,
            include_str!("../../dev-Cargo.toml"),
            b"../",
        )?;
    } else {
        let current_cargo_toml =
            fs::read_to_string("Cargo.toml").context("Failed to read the file `Cargo.toml`")?;
        check_cargo_toml(&info_file.exercises, &current_cargo_toml, b"")?;
    }

    check_exercises(&info_file)?;
    check_solutions(&info_file)?;

    println!("\nEverything looks fine!");

    Ok(())
}
