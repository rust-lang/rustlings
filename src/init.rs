use anyhow::{bail, Context, Result};
use crossterm::style::Stylize;
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    io::ErrorKind,
    path::Path,
    process::{Command, Stdio},
};

use crate::{cargo_toml::updated_cargo_toml, embedded::EMBEDDED_FILES, info_file::InfoFile};

pub fn init() -> Result<()> {
    // Prevent initialization in a directory that contains the file `Cargo.toml`.
    // This can mean that Rustlings was already initialized in this directory.
    // Otherwise, this can cause problems with Cargo workspaces.
    if Path::new("Cargo.toml").exists() {
        bail!(CARGO_TOML_EXISTS_ERR);
    }

    let rustlings_path = Path::new("rustlings");
    if let Err(e) = create_dir(rustlings_path) {
        if e.kind() == ErrorKind::AlreadyExists {
            bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
        }
        return Err(e.into());
    }

    set_current_dir("rustlings")
        .context("Failed to change the current directory to `rustlings/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    create_dir("solutions").context("Failed to create the `solutions/` directory")?;
    for dir in EMBEDDED_FILES.exercise_dirs {
        let mut dir_path = String::with_capacity(10 + dir.name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir(&dir_path)
            .with_context(|| format!("Failed to create the directory {dir_path}"))?;
    }
    for exercise_info in &info_file.exercises {
        let solution_path = exercise_info.sol_path();
        fs::write(&solution_path, INIT_SOLUTION_FILE)
            .with_context(|| format!("Failed to create the file {solution_path}"))?;
    }

    let current_cargo_toml = include_str!("../dev-Cargo.toml");
    // Skip the first line (comment).
    let newline_ind = current_cargo_toml
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("The embedded `Cargo.toml` is empty or contains only one line")?;
    let current_cargo_toml = current_cargo_toml
        .get(newline_ind + 1..)
        .context("The embedded `Cargo.toml` contains only one line")?;
    let updated_cargo_toml = updated_cargo_toml(&info_file.exercises, current_cargo_toml, b"")
        .context("Failed to generate `Cargo.toml`")?;
    fs::write("Cargo.toml", updated_cargo_toml)
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `rustlings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    // Ignore any Git error because Git initialization is not required.
    let _ = Command::new("git")
        .arg("init")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!(
        "\n{}\n\n{}",
        "Initialization done ✓".green(),
        POST_INIT_MSG.bold(),
    );

    Ok(())
}

const INIT_SOLUTION_FILE: &[u8] = b"fn main() {
    // DON'T EDIT THIS SOLUTION FILE!
    // It will be automatically filled after you finish the exercise.
}
";

const GITIGNORE: &[u8] = b".rustlings-state.txt
solutions
Cargo.lock
target
.vscode
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const CARGO_TOML_EXISTS_ERR: &str = "The current directory contains the file `Cargo.toml`.

If you already initialized Rustlings, run the command `rustlings` for instructions on getting started with the exercises.
Otherwise, please run `rustlings init` again in another directory.";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustlings` already exists in the current directory.
You probably already initialized Rustlings.
Run `cd rustlings`
Then run `rustlings` again";

const POST_INIT_MSG: &str = "Run `cd rustlings` to go into the generated directory.
Then run `rustlings` to get started.";
