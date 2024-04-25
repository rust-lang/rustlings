use anyhow::{bail, Context, Result};
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    path::Path,
    process::Command,
};

use crate::CURRENT_FORMAT_VERSION;

fn create_rel_dir(dir_name: &str, current_dir: &str) -> Result<()> {
    create_dir(dir_name)
        .with_context(|| format!("Failed to create the directory {current_dir}/{dir_name}"))?;
    println!("Created the directory {current_dir}/{dir_name}");
    Ok(())
}

fn write_rel_file<C>(file_name: &str, current_dir: &str, content: C) -> Result<()>
where
    C: AsRef<[u8]>,
{
    fs::write(file_name, content)
        .with_context(|| format!("Failed to create the file {current_dir}/{file_name}"))?;
    // Space to align with `create_rel_dir`.
    println!("Created the file      {current_dir}/{file_name}");
    Ok(())
}

pub fn new(path: &Path, no_git: bool) -> Result<()> {
    let dir_name = path.to_string_lossy();

    create_dir(path).with_context(|| format!("Failed to create the directory {dir_name}"))?;
    println!("Created the directory {dir_name}");

    set_current_dir(path)
        .with_context(|| format!("Failed to set {dir_name} as the current directory"))?;

    if !no_git
        && !Command::new("git")
            .arg("init")
            .status()
            .context("Failed to run `git init`")?
            .success()
    {
        bail!("`git init` didn't run successfully. See the error message above");
    }

    write_rel_file(".gitignore", &dir_name, crate::init::GITIGNORE)?;

    create_rel_dir("exercises", &dir_name)?;
    create_rel_dir("solutions", &dir_name)?;

    write_rel_file(
        "info.toml",
        &dir_name,
        format!("{INFO_FILE_BEFORE_FORMAT_VERSION}{CURRENT_FORMAT_VERSION}{INFO_FILE_AFTER_FORMAT_VERSION}"),
    )?;

    write_rel_file("Cargo.toml", &dir_name, CARGO_TOML)?;

    write_rel_file("README.md", &dir_name, README)?;

    create_rel_dir(".vscode", &dir_name)?;
    write_rel_file(
        ".vscode/extensions.json",
        &dir_name,
        crate::init::VS_CODE_EXTENSIONS_JSON,
    )?;

    println!("\nInitialization done ✓");

    Ok(())
}

const INFO_FILE_BEFORE_FORMAT_VERSION: &str =
    "# The format version is an indicator of the compatibility of third-party exercises with the
# Rustlings program.
# The format version is not the same as the version of the Rustlings program.
# In case Rustlings makes an unavoidable breaking change to the expected format of third-party
# exercises, you would need to raise this version and adapt to the new format.
# Otherwise, the newest version of the Rustlings program won't be able to run these exercises.
format_version = ";

const INFO_FILE_AFTER_FORMAT_VERSION: &str = r#"

# Optional multi-line message to be shown to users when just starting with the exercises.
welcome_message = """Welcome to these third-party Rustlings exercises."""

# Optional multi-line message to be shown to users after finishing all exercises.
final_message = """We hope that you found the exercises helpful :D"""

# Repeat this section for every exercise.
[[exercises]]
# Exercise name which is the exercise file name without the `.rs` extension.
name = "???"

# Optional directory name to be provided if you want to organize exercises in directories.
# If `dir` is specified, the exercise path is `exercises/DIR/NAME.rs`
# Otherwise, the path is `exercises/NAME.rs`
# dir = "???"

# Rustlings expects the exercise to contain tests and run them.
# You can optionally disable testing by setting `test` to `false` (the default is `true`).
# In that case, the exercise will be considered done when it just successfully compiles.
# test = true

# Rustlings will always run Clippy on exercises.
# You can optionally set `strict_clippy` to `true` (the default is `false`) to only consider
# the exercise as done when there are no warnings left.
# strict_clippy = false

# A multi-line hint to be shown to users on request.
hint = """???"""
"#;

const CARGO_TOML: &[u8] =
    br#"# Don't edit the `bin` list manually! It is updated by `rustlings dev update`
bin = []

[package]
name = "rustlings"
edition = "2021"
publish = false

[dependencies]
"#;

const README: &str = "# Rustlings 🦀

Welcome to these third-party Rustlings exercises 😃

First,
[install Rustlings using the official instructions in the README of the Rustlings project](https://github.com/rust-lang/rustlings) ✅

Then, open your terminal in this directory and run `rustlings` to get started with the exercises 🚀
";
