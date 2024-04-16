use std::fs::{self, create_dir};

use anyhow::{Context, Result};

use crate::CURRENT_FORMAT_VERSION;

pub fn init() -> Result<()> {
    create_dir("rustlings").context("Failed to create the directory `rustlings`")?;

    create_dir("rustlings/exercises")
        .context("Failed to create the directory `rustlings/exercises`")?;

    create_dir("rustlings/solutions")
        .context("Failed to create the directory `rustlings/solutions`")?;

    fs::write(
        "rustlings/info.toml",
        format!("{INFO_FILE_BEFORE_FORMAT_VERSION}{CURRENT_FORMAT_VERSION}{INFO_FILE_AFTER_FORMAT_VERSION}"),
    )
    .context("Failed to create the file `rustlings/info.toml`")?;

    fs::write(
        "rustlings/Cargo.toml",
        format!("{CARGO_TOML_COMMENT}{}", crate::init::CARGO_TOML_PACKAGE),
    )
    .context("Failed to create the file `rustlings/Cargo.toml`")?;

    fs::write("rustlings/.gitignore", crate::init::GITIGNORE)
        .context("Failed to create the file `rustlings/.gitignore`")?;

    fs::write("rustlings/README.md", README)
        .context("Failed to create the file `rustlings/README.md`")?;

    create_dir("rustlings/.vscode")
        .context("Failed to create the directory `rustlings/.vscode`")?;
    fs::write(
        "rustlings/.vscode/extensions.json",
        crate::init::VS_CODE_EXTENSIONS_JSON,
    )
    .context("Failed to create the file `rustlings/.vscode/extensions.json`")?;

    println!("{INIT_DONE}");

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

# A mutli-line hint to be shown to users on request.
hint = """???"""
"#;

const CARGO_TOML_COMMENT: &str =
    "# You shouldn't edit this file manually! It is updated by `rustlings dev check`

";

const README: &str = "# Rustlings 🦀

Welcome to these third-party Rustlings exercises 😃

First,
[install Rustlings using the official instructions in the README of the Rustlings project](https://github.com/rust-lang/rustlings) ✅

Then, open your terminal in this directory and run `rustlings` to get started with the exercises 🚀
";

const INIT_DONE: &str = r#"Initialization done!
You can start developing third-party Rustlings exercises in the `rustlings` directory :D

If the initialization was done in a Rust project which is a Cargo workspace, you need to add the
path to the `rustlings` directory to the `workspace.exclude` list in the project's `Cargo.toml`
file. For example:

[workspace]
exclude = ["rustlings"]"#;
