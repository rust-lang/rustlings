use anyhow::{bail, Context, Result};
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    io::ErrorKind,
    path::Path,
};

use crate::{embedded::EMBEDDED_FILES, info_file::ExerciseInfo};

fn cargo_toml(exercise_infos: &[ExerciseInfo]) -> Vec<u8> {
    let mut cargo_toml = Vec::with_capacity(1 << 13);
    cargo_toml.extend_from_slice(b"bin = [\n");
    for exercise_info in exercise_infos {
        cargo_toml.extend_from_slice(b"  { name = \"");
        cargo_toml.extend_from_slice(exercise_info.name.as_bytes());
        cargo_toml.extend_from_slice(b"\", path = \"exercises/");
        if let Some(dir) = &exercise_info.dir {
            cargo_toml.extend_from_slice(dir.as_bytes());
            cargo_toml.push(b'/');
        }
        cargo_toml.extend_from_slice(exercise_info.name.as_bytes());
        cargo_toml.extend_from_slice(b".rs\" },\n");
    }

    cargo_toml.extend_from_slice(b"]\n\n");
    cargo_toml.extend_from_slice(CARGO_TOML_PACKAGE.as_bytes());

    cargo_toml
}

pub fn init(exercise_infos: &[ExerciseInfo]) -> Result<()> {
    if Path::new("exercises").is_dir() && Path::new("Cargo.toml").is_file() {
        bail!(PROBABLY_IN_RUSTLINGS_DIR_ERR);
    }

    let rustlings_path = Path::new("rustlings");
    if let Err(e) = create_dir(rustlings_path) {
        if e.kind() == ErrorKind::AlreadyExists {
            bail!(RUSTLINGS_DIR_ALREADY_EXISTS_ERR);
        }
        return Err(e.into());
    }

    set_current_dir("rustlings")
        .context("Failed to change the current directory to `rustlings`")?;

    EMBEDDED_FILES
        .init_exercises_dir()
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    fs::write("Cargo.toml", cargo_toml(exercise_infos))
        .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `rustlings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    Ok(())
}

pub const CARGO_TOML_PACKAGE: &str = r#"[package]
name = "rustlings"
edition = "2021"
publish = false
"#;

pub const GITIGNORE: &[u8] = b"Cargo.lock
.rustlings-state.txt
target
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const PROBABLY_IN_RUSTLINGS_DIR_ERR: &str =
    "A directory with the name `exercises` and a file with the name `Cargo.toml` already exist
in the current directory. It looks like Rustlings was already initialized here.
Run `rustlings` for instructions on getting started with the exercises.

If you didn't already initialize Rustlings, please initialize it in another directory.";

const RUSTLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `rustlings` already exists in the current directory.
You probably already initialized Rustlings.
Run `cd rustlings`
Then run `rustlings` again";
