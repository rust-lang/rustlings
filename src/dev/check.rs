use std::fs;

use anyhow::{Context, Result};

use crate::{info_file::InfoFile, init::cargo_toml};

pub fn check(info_file: InfoFile) -> Result<()> {
    // TODO: Add checks

    fs::write("Cargo.toml", cargo_toml(&info_file.exercises))
        .context("Failed to update the file `Cargo.toml`")?;

    println!("Everything looks fine!");

    Ok(())
}
