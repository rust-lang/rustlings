use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::exercise::Exercise;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize, Deserialize)]
pub struct RustAnalyzerProject {
    sysroot_src: String,
    crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize)]
pub struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<String>,
    cfg: Vec<String>,
}

impl RustAnalyzerProject {
    pub fn build() -> Result<Self> {
        // check if RUST_SRC_PATH is set
        if let Ok(sysroot_src) = env::var("RUST_SRC_PATH") {
            return Ok(Self {
                sysroot_src,
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
        let Ok(sysroot_src) = sysroot_src.into_os_string().into_string() else {
            bail!("The sysroot path is invalid UTF8");
        };

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
                root_module: exercise.path.display().to_string(),
                edition: "2021".to_string(),
                deps: Vec::new(),
                // This allows rust_analyzer to work inside #[test] blocks
                cfg: vec!["test".to_string()],
            })
            .collect();
        Ok(())
    }
}
