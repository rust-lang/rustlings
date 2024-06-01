use anyhow::{Context, Result};
use std::{io::Read, path::Path, process::Command};

/// Run a command with a description for a possible error and append the merged stdout and stderr.
/// The boolean in the returned `Result` is true if the command's exit status is success.
pub fn run_cmd(mut cmd: Command, description: &str, output: &mut Vec<u8>) -> Result<bool> {
    let (mut reader, writer) = os_pipe::pipe()
        .with_context(|| format!("Failed to create a pipe to run the command `{description}``"))?;

    let writer_clone = writer.try_clone().with_context(|| {
        format!("Failed to clone the pipe writer for the command `{description}`")
    })?;

    let mut handle = cmd
        .stdout(writer_clone)
        .stderr(writer)
        .spawn()
        .with_context(|| format!("Failed to run the command `{description}`"))?;

    // Prevent pipe deadlock.
    drop(cmd);

    reader
        .read_to_end(output)
        .with_context(|| format!("Failed to read the output of the command `{description}`"))?;

    output.push(b'\n');

    handle
        .wait()
        .with_context(|| format!("Failed to wait on the command `{description}` to exit"))
        .map(|status| status.success())
}

pub struct CargoCmd<'a> {
    pub subcommand: &'a str,
    pub args: &'a [&'a str],
    pub bin_name: &'a str,
    pub description: &'a str,
    /// RUSTFLAGS="-A warnings"
    pub hide_warnings: bool,
    /// Added as `--target-dir` if `Self::dev` is true.
    pub target_dir: &'a Path,
    /// The output buffer to append the merged stdout and stderr.
    pub output: &'a mut Vec<u8>,
    /// true while developing Rustlings.
    pub dev: bool,
}

impl<'a> CargoCmd<'a> {
    /// Run `cargo SUBCOMMAND --bin EXERCISE_NAME … ARGS`.
    pub fn run(&mut self) -> Result<bool> {
        let mut cmd = Command::new("cargo");
        cmd.arg(self.subcommand);

        // A hack to make `cargo run` work when developing Rustlings.
        if self.dev {
            cmd.arg("--manifest-path")
                .arg("dev/Cargo.toml")
                .arg("--target-dir")
                .arg(self.target_dir);
        }

        cmd.arg("--color")
            .arg("always")
            .arg("-q")
            .arg("--bin")
            .arg(self.bin_name)
            .args(self.args);

        if self.hide_warnings {
            cmd.env("RUSTFLAGS", "-A warnings");
        }

        run_cmd(cmd, self.description, self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cmd() {
        let mut cmd = Command::new("echo");
        cmd.arg("Hello");

        let mut output = Vec::with_capacity(8);
        run_cmd(cmd, "echo …", &mut output).unwrap();

        assert_eq!(output, b"Hello\n\n");
    }
}
