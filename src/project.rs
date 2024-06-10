use anyhow::{Context, Result};
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::exercise::Exercise;

/// 包含生成 rust-project.json 文件所需的結構和構建數據的函數
#[derive(Serialize)]
struct RustAnalyzerProject {
    sysroot_src: PathBuf,
    crates: Vec<Crate>,
}

#[derive(Serialize)]
struct Crate {
    root_module: PathBuf,
    edition: &'static str,
    // 未使用，但在 JSON 文件中是必需的。
    deps: Vec<()>,
    // 僅使用 `test` 進行所有 crates。
    // 因此，使用數組而不是 `Vec`。
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
                // 這允許 rust_analyzer 在 `#[test]` 區塊內工作
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
            .context("無法從 `rustc` 獲取 sysroot。您是否安裝了 `rustc`？")?
            .stdout;

        let toolchain =
            String::from_utf8(toolchain).context("工具鏈路徑是無效的 UTF8")?;
        let toolchain = toolchain.trim_end();
        println!("確定的工具鏈: {toolchain}\n");

        let mut sysroot_src = PathBuf::with_capacity(256);
        sysroot_src.extend([toolchain, "lib", "rustlib", "src", "rust", "library"]);

        Ok(Self {
            sysroot_src,
            crates,
        })
    }
}

/// 將 `rust-project.json` 寫入磁碟。
pub fn write_project_json(exercises: Vec<Exercise>) -> Result<()> {
    let content = RustAnalyzerProject::build(exercises)?;

    // 使用容量 2^14，因為文件長度以字節為單位高於 2^13。
    // 最終長度不確定，因為它取決於用戶的 sysroot 路徑、
    // 當前的練習數量等。
    let mut buf = Vec::with_capacity(1 << 14);
    serde_json::to_writer(&mut buf, &content)?;
    std::fs::write("rust-project.json", buf)?;

    Ok(())
}
