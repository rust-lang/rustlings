use glob::glob;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RustProject {
    sysroot_src: String,
    crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize)]
struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<String>,
}

impl RustProject {
    fn new() -> RustProject {
        RustProject {
            sysroot_src: RustProject::get_sysroot_src(),
            crates: Vec::new(),
        }
    }

    fn get_sysroot_src() -> String {
        let mut sysroot_src = home::rustup_home()
            .expect("Can't find Rustup... aborting")
            .to_string_lossy()
            .to_string();

        use std::process::Command;
        let output = Command::new("rustup")
            .arg("default")
            .output()
            .expect("Failed to get rustup default toolchain");

        let toolchain = String::from_utf8_lossy(&output.stdout).to_string();

        sysroot_src += "/toolchains/";
        sysroot_src += toolchain
            .split_once(' ')
            .expect("Malformed default toolchain path")
            .0;
        sysroot_src += "/lib/rustlib/src/rust/library";
        println!("{}", sysroot_src);
        sysroot_src
    }
}

fn main() {
    let mut project = RustProject::new();
    for e in glob("./exercises/**/*").expect("Glob failed to read pattern") {
        let path = e
            .expect("Unable to extract path")
            .to_string_lossy()
            .to_string();
        if let Some((_, ext)) = path.split_once(".") {
            if ext == "rs" {
                project.crates.push(Crate {
                    deps: Vec::new(),
                    edition: "2021".to_string(),
                    root_module: path,
                })
            }
        }
    }
    std::fs::write(
        "./rust-project.json",
        serde_json::to_vec(&project).expect("Failed to serialize to JSON"),
    )
    .expect("Failed to write file");
}
