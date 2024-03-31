use anyhow::{bail, Context, Result};
use std::{
    env::set_current_dir,
    fs::{create_dir, OpenOptions},
    io::{self, ErrorKind, Write},
    path::Path,
};

use crate::{embedded::EMBEDDED_FILES, exercise::Exercise};

fn create_cargo_toml(exercises: &[Exercise]) -> io::Result<()> {
    let mut cargo_toml = Vec::with_capacity(1 << 13);
    cargo_toml.extend_from_slice(
        br#"[package]
name = "rustlings"
version = "0.0.0"
edition = "2021"
publish = false
"#,
    );
    for exercise in exercises {
        cargo_toml.extend_from_slice(b"\n[[bin]]\nname = \"");
        cargo_toml.extend_from_slice(exercise.name.as_bytes());
        cargo_toml.extend_from_slice(b"\"\npath = \"");
        cargo_toml.extend_from_slice(exercise.path.to_str().unwrap().as_bytes());
        cargo_toml.extend_from_slice(b"\"\n");
    }
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("Cargo.toml")?
        .write_all(&cargo_toml)
}

fn create_gitignore() -> io::Result<()> {
    let gitignore = b"/target";
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(".gitignore")?
        .write_all(gitignore)
}

fn create_vscode_dir() -> Result<()> {
    create_dir(".vscode").context("Failed to create the directory `.vscode`")?;
    let vs_code_extensions_json = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(".vscode/extensions.json")?
        .write_all(vs_code_extensions_json)?;

    Ok(())
}

pub fn init_rustlings(exercises: &[Exercise]) -> Result<()> {
    if Path::new("exercises").is_dir() && Path::new("Cargo.toml").is_file() {
        bail!(
            "A directory with the name `exercises` and a file with the name `Cargo.toml` already exist
in the current directory. It looks like Rustlings was already initialized here.
Run `rustlings` for instructions on getting started with the exercises.

If you didn't already initialize Rustlings, please initialize it in another directory."
        );
    }

    let rustlings_path = Path::new("rustlings");
    if let Err(e) = create_dir(rustlings_path) {
        if e.kind() == ErrorKind::AlreadyExists {
            bail!(
                "A directory with the name `rustlings` already exists in the current directory.
You probably already initialized Rustlings.
Run `cd rustlings`
Then run `rustlings` again"
            );
        }
        return Err(e.into());
    }

    set_current_dir("rustlings")
        .context("Failed to change the current directory to `rustlings`")?;

    EMBEDDED_FILES
        .init_exercises_dir()
        .context("Failed to initialize the `rustlings/exercises` directory")?;

    create_cargo_toml(exercises).context("Failed to create the file `rustlings/Cargo.toml`")?;

    create_gitignore().context("Failed to create the file `rustlings/.gitignore`")?;

    create_vscode_dir().context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    Ok(())
}
