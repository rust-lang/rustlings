use anyhow::{anyhow, bail, Context, Error, Result};
use std::{
    cmp::Ordering,
    fs::{self, read_dir, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use crate::{
    cargo_toml::{append_bins, bins_start_end_ind, BINS_BUFFER_CAPACITY},
    cmd::CmdRunner,
    collections::{hash_set_with_capacity, HashSet},
    exercise::{RunnableExercise, OUTPUT_CAPACITY},
    info_file::{ExerciseInfo, InfoFile},
    CURRENT_FORMAT_VERSION,
};

// Find a char that isn't allowed in the exercise's `name` or `dir`.
fn forbidden_char(input: &str) -> Option<char> {
    input.chars().find(|c| !c.is_alphanumeric() && *c != '_')
}

// Check that the `Cargo.toml` file is up-to-date.
fn check_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    cargo_toml_path: &str,
    exercise_path_prefix: &[u8],
) -> Result<()> {
    let current_cargo_toml = fs::read_to_string(cargo_toml_path)
        .with_context(|| format!("Failed to read the file `{cargo_toml_path}`"))?;

    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(&current_cargo_toml)?;

    let old_bins = &current_cargo_toml.as_bytes()[bins_start_ind..bins_end_ind];
    let mut new_bins = Vec::with_capacity(BINS_BUFFER_CAPACITY);
    append_bins(&mut new_bins, exercise_infos, exercise_path_prefix);

    if old_bins != new_bins {
        if cfg!(debug_assertions) {
            bail!("The file `dev/Cargo.toml` is outdated. Please run `cargo run -- dev update` to update it. Then run `cargo run -- dev check` again");
        }

        bail!("The file `Cargo.toml` is outdated. Please run `rustlings dev update` to update it. Then run `rustlings dev check` again");
    }

    Ok(())
}

// Check the info of all exercises and return their paths in a set.
fn check_info_file_exercises(info_file: &InfoFile) -> Result<HashSet<PathBuf>> {
    let mut names = hash_set_with_capacity(info_file.exercises.len());
    let mut paths = hash_set_with_capacity(info_file.exercises.len());

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

        if exercise_info.hint.trim_ascii().is_empty() {
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

        if !file_buf.contains("// TODO") {
            bail!("Didn't find any `// TODO` comment in the file `{path}`.\nYou need to have at least one such comment to guide the user.");
        }

        if !exercise_info.test && file_buf.contains("#[test]") {
            bail!("The file `{path}` contains tests annotated with `#[test]` but the exercise `{name}` has `test = false` in the `info.toml` file");
        }

        file_buf.clear();

        paths.insert(PathBuf::from(path));
    }

    Ok(paths)
}

// Check `dir` for unexpected files.
// Only Rust files in `allowed_rust_files` and `README.md` files are allowed.
// Only one level of directory nesting is allowed.
fn check_unexpected_files(dir: &str, allowed_rust_files: &HashSet<PathBuf>) -> Result<()> {
    let unexpected_file = |path: &Path| {
        anyhow!("Found the file `{}`. Only `README.md` and Rust files related to an exercise in `info.toml` are allowed in the `{dir}` directory", path.display())
    };

    for entry in read_dir(dir).with_context(|| format!("Failed to open the `{dir}` directory"))? {
        let entry = entry.with_context(|| format!("Failed to read the `{dir}` directory"))?;

        if entry.file_type().unwrap().is_file() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            if file_name == "README.md" {
                continue;
            }

            if !allowed_rust_files.contains(&path) {
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

            if !allowed_rust_files.contains(&path) {
                return Err(unexpected_file(&path));
            }
        }
    }

    Ok(())
}

fn check_exercises_unsolved(info_file: &InfoFile, cmd_runner: &CmdRunner) -> Result<()> {
    println!(
        "Running all exercises to check that they aren't already solved. This may take a while…\n",
    );
    thread::scope(|s| {
        let handles = info_file
            .exercises
            .iter()
            .filter_map(|exercise_info| {
                if exercise_info.skip_check_unsolved {
                    return None;
                }

                Some((
                    exercise_info.name.as_str(),
                    s.spawn(|| exercise_info.run_exercise(None, cmd_runner)),
                ))
            })
            .collect::<Vec<_>>();

        for (exercise_name, handle) in handles {
            let Ok(result) = handle.join() else {
                bail!("Panic while trying to run the exericse {exercise_name}");
            };

            match result {
                Ok(true) => bail!(
                    "The exercise {exercise_name} is already solved.\n{SKIP_CHECK_UNSOLVED_HINT}",
                ),
                Ok(false) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    })
}

fn check_exercises(info_file: &InfoFile, cmd_runner: &CmdRunner) -> Result<()> {
    match info_file.format_version.cmp(&CURRENT_FORMAT_VERSION) {
        Ordering::Less => bail!("`format_version` < {CURRENT_FORMAT_VERSION} (supported version)\nPlease migrate to the latest format version"),
        Ordering::Greater => bail!("`format_version` > {CURRENT_FORMAT_VERSION} (supported version)\nTry updating the Rustlings program"),
        Ordering::Equal => (),
    }

    let info_file_paths = check_info_file_exercises(info_file)?;
    let handle = thread::spawn(move || check_unexpected_files("exercises", &info_file_paths));

    check_exercises_unsolved(info_file, cmd_runner)?;
    handle.join().unwrap()
}

enum SolutionCheck {
    Success { sol_path: String },
    MissingRequired,
    MissingOptional,
    RunFailure { output: Vec<u8> },
    Err(Error),
}

fn check_solutions(
    require_solutions: bool,
    info_file: &InfoFile,
    cmd_runner: &CmdRunner,
) -> Result<()> {
    println!("Running all solutions. This may take a while…\n");
    thread::scope(|s| {
        let handles = info_file
            .exercises
            .iter()
            .map(|exercise_info| {
                s.spawn(|| {
                    let sol_path = exercise_info.sol_path();
                    if !Path::new(&sol_path).exists() {
                        if require_solutions {
                            return SolutionCheck::MissingRequired;
                        }

                        return SolutionCheck::MissingOptional;
                    }

                    let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
                    match exercise_info.run_solution(Some(&mut output), cmd_runner) {
                        Ok(true) => SolutionCheck::Success { sol_path },
                        Ok(false) => SolutionCheck::RunFailure { output },
                        Err(e) => SolutionCheck::Err(e),
                    }
                })
            })
            .collect::<Vec<_>>();

        let mut sol_paths = hash_set_with_capacity(info_file.exercises.len());
        let mut fmt_cmd = Command::new("rustfmt");
        fmt_cmd
            .arg("--check")
            .arg("--edition")
            .arg("2021")
            .arg("--color")
            .arg("always")
            .stdin(Stdio::null());

        for (exercise_info, handle) in info_file.exercises.iter().zip(handles) {
            let Ok(check_result) = handle.join() else {
                bail!(
                    "Panic while trying to run the solution of the exericse {}",
                    exercise_info.name,
                );
            };

            match check_result {
                SolutionCheck::Success { sol_path } => {
                    fmt_cmd.arg(&sol_path);
                    sol_paths.insert(PathBuf::from(sol_path));
                }
                SolutionCheck::MissingRequired => {
                    bail!(
                        "The solution of the exercise {} is missing",
                        exercise_info.name,
                    );
                }
                SolutionCheck::MissingOptional => (),
                SolutionCheck::RunFailure { output } => {
                    io::stderr().lock().write_all(&output)?;
                    bail!(
                        "Running the solution of the exercise {} failed with the error above",
                        exercise_info.name,
                    );
                }
                SolutionCheck::Err(e) => return Err(e),
            }
        }

        let handle = s.spawn(move || check_unexpected_files("solutions", &sol_paths));

        if !fmt_cmd
            .status()
            .context("Failed to run `rustfmt` on all solution files")?
            .success()
        {
            bail!("Some solutions aren't formatted. Run `rustfmt` on them");
        }

        handle.join().unwrap()
    })
}

pub fn check(require_solutions: bool) -> Result<()> {
    let info_file = InfoFile::parse()?;

    if cfg!(debug_assertions) {
        // A hack to make `cargo run -- dev check` work when developing Rustlings.
        check_cargo_toml(&info_file.exercises, "dev/Cargo.toml", b"../")?;
    } else {
        check_cargo_toml(&info_file.exercises, "Cargo.toml", b"")?;
    }

    let cmd_runner = CmdRunner::build()?;
    check_exercises(&info_file, &cmd_runner)?;
    check_solutions(require_solutions, &info_file, &cmd_runner)?;

    println!("Everything looks fine!");

    Ok(())
}

const SKIP_CHECK_UNSOLVED_HINT: &str = "If this is an introduction exercise that is intended to be already solved, add `skip_check_unsolved = true` to the exercise's metadata in the `info.toml` file";
