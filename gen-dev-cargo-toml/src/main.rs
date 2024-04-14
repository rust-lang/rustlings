// Generates `dev/Cargo.toml` such that it is synced with `info.toml`.
// `dev/Cargo.toml` is a hack to allow using `cargo run` to test `rustlings`
// during development.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    fs::{self, create_dir},
    io::ErrorKind,
};

#[derive(Deserialize)]
struct ExerciseInfo {
    name: String,
    dir: Option<String>,
}

#[derive(Deserialize)]
struct InfoFile {
    exercises: Vec<ExerciseInfo>,
}

fn main() -> Result<()> {
    let exercise_infos = toml_edit::de::from_str::<InfoFile>(
        &fs::read_to_string("info.toml").context("Failed to read `info.toml`")?,
    )
    .context("Failed to deserialize `info.toml`")?
    .exercises;

    let mut buf = Vec::with_capacity(1 << 14);

    buf.extend_from_slice(
        b"# This file is a hack to allow using `cargo run` to test `rustlings` during development.
# You shouldn't edit it manually. It is created and updated by running `cargo run -p gen-dev-cargo-toml`.

bin = [\n",
    );

    for exercise_info in exercise_infos {
        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b"\", path = \"../exercises/");
        if let Some(dir) = &exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");
    }

    buf.extend_from_slice(
        br#"]

[package]
name = "rustlings-dev"
edition = "2021"
publish = false
"#,
    );

    if let Err(e) = create_dir("dev") {
        if e.kind() != ErrorKind::AlreadyExists {
            bail!("Failed to create the `dev` directory: {e}");
        }
    }

    fs::write("dev/Cargo.toml", buf).context("Failed to write `dev/Cargo.toml`")
}
