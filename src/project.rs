use anyhow::{Context, Result};
use serde::Serialize;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::exercise::Exercise;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize)]
pub struct RustAnalyzerProject {
    sysroot_src: PathBuf,
    crates: Vec<Crate>,
}

#[derive(Serialize)]
struct Crate {
    root_module: PathBuf,
    edition: &'static str,
    // Not used, but required in the JSON file.
    deps: Vec<()>,
    cfg: [&'static str; 1],
}

impl RustAnalyzerProject {
    pub fn build() -> Result<Self> {
        // check if RUST_SRC_PATH is set
        if let Some(path) = env::var_os("RUST_SRC_PATH") {
            return Ok(Self {
                sysroot_src: PathBuf::from(path),
                crates: Vec::new(),
            });
        }

        let toolchain = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .stderr(Stdio::inherit())
            .output()
            .context("Failed to get the sysroot from `rustc`. Do you have `rustc` installed?")?
            .stdout;

        let toolchain =
            String::from_utf8(toolchain).context("The toolchain path is invalid UTF8")?;
        let toolchain = toolchain.trim_end();

        println!("Determined toolchain: {toolchain}\n");

        let mut sysroot_src = PathBuf::with_capacity(256);
        sysroot_src.extend([toolchain, "lib", "rustlib", "src", "rust", "library"]);

        Ok(Self {
            sysroot_src,
            crates: Vec::new(),
        })
    }

    /// Write rust-project.json to disk
    pub fn write_to_disk(&self) -> Result<(), std::io::Error> {
        // Using the capacity 2^14 = 16384 since the file length in bytes is higher than 2^13.
        // The final length is not known exactly because it depends on the user's sysroot path,
        // the current number of exercises etc.
        let mut buf = Vec::with_capacity(16384);
        serde_json::to_writer(&mut buf, &self).expect("Failed to serialize to JSON");
        std::fs::write("rust-project.json", buf)?;
        Ok(())
    }

    /// Parse the exercises folder for .rs files, any matches will create
    /// a new `crate` in rust-project.json which allows rust-analyzer to
    /// treat it like a normal binary
    pub fn exercises_to_json(&mut self, exercises: Vec<Exercise>) -> Result<(), Box<dyn Error>> {
        self.crates = exercises
            .into_iter()
            .map(|exercise| Crate {
                root_module: exercise.path,
                edition: "2021",
                deps: Vec::new(),
                // This allows rust_analyzer to work inside #[test] blocks
                cfg: ["test"],
            })
            .collect();
        Ok(())
    }
}
