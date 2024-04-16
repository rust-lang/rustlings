use std::fs;

use anyhow::{Context, Result};

use crate::{info_file::InfoFile, init::cargo_toml};

pub fn check(info_file: InfoFile) -> Result<()> {
    // TODO: Add checks

    // TODO: Keep dependencies!
    fs::write("Cargo.toml", cargo_toml(&info_file.exercises))
        .context("Failed to update the file `Cargo.toml`")?;
    println!("Updated `Cargo.toml`");

    println!("\nEverything looks fine!");

    Ok(())
}
