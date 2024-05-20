use anyhow::{Context, Result};
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::exercise::Exercise;

/// Contains the structure of resulting rust-project.json file
/// and functions to build the data required to create the file
#[derive(Serialize)]
struct RustAnalyzerProject {
    sysroot_src: PathBuf,
    crates: Vec<Crate>,
}

#[derive(Serialize)]
struct Crate {
    root_module: PathBuf,
    edition: &'static str,
    // Not used, but required in the JSON file.
    deps: Vec<()>,
    // Only `test` is used for all crates.
    // Therefore, an array is used instead of a `Vec`.
    cfg: [&'static str; 1],
}

impl RustAnalyzerProject {
    fn build(exercises: Vec<Exercise>) -> Result<Self> {
        let crates = exercises
            .into_iter()
            .map(|exercise| Crate {
                root_module: exercise.path,
                edition: "2021",
                deps: Vec::new(),
                // This allows rust_analyzer to work inside `#[test]` blocks
                cfg: ["test"],
            })
            .collect();

        if let Some(path) = env::var_os("RUST_SRC_PATH") {
            return Ok(Self {
                sysroot_src: PathBuf::from(path),
                crates,
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
            crates,
        })
    }
}

/// Write `rust-project.json` to disk.
pub fn write_project_json(exercises: Vec<Exercise>) -> Result<()> {
    let content = RustAnalyzerProject::build(exercises)?;

    // Using the capacity 2^14 since the file length in bytes is higher than 2^13.
    // The final length is not known exactly because it depends on the user's sysroot path,
    // the current number of exercises etc.
    let mut buf = Vec::with_capacity(1 << 14);
    serde_json::to_writer(&mut buf, &content)?;
    std::fs::write("rust-project.json", buf)?;

    Ok(())
}
