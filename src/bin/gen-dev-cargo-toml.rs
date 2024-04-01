// Generates `dev/Cargo.toml` such that it is synced with `info.toml`.
// `dev/Cargo.toml` is a hack to allow using `cargo r` to test `rustlings`
// during development.

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    fs::{self, create_dir},
    io::ErrorKind,
};

#[derive(Deserialize)]
struct Exercise {
    name: String,
    path: String,
}

#[derive(Deserialize)]
struct InfoToml {
    exercises: Vec<Exercise>,
}

fn main() -> Result<()> {
    let exercises = toml_edit::de::from_str::<InfoToml>(
        &fs::read_to_string("info.toml").context("Failed to read `info.toml`")?,
    )
    .context("Failed to deserialize `info.toml`")?
    .exercises;

    let mut buf = Vec::with_capacity(1 << 14);

    buf.extend_from_slice(
        b"# This file is a hack to allow using `cargo r` to test `rustlings` during development.
# You shouldn't edit it manually. It is created and updated by running `cargo run --bin gen-dev-cargo-toml`.

bin = [\n",
    );

    for exercise in exercises {
        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise.name.as_bytes());
        buf.extend_from_slice(b"\", path = \"../");
        buf.extend_from_slice(exercise.path.as_bytes());
        buf.extend_from_slice(b"\" },\n");
    }

    buf.extend_from_slice(
        br#"]

[package]
name = "rustlings"
version = "0.0.0"
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
