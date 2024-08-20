use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

/// Run a command with a description for a possible error and append the merged stdout and stderr.
/// The boolean in the returned `Result` is true if the command's exit status is success.
fn run_cmd(mut cmd: Command, description: &str, output: Option<&mut Vec<u8>>) -> Result<bool> {
    let spawn = |mut cmd: Command| {
        // NOTE: The closure drops `cmd` which prevents a pipe deadlock.
        cmd.stdin(Stdio::null())
            .spawn()
            .with_context(|| format!("Failed to run the command `{description}`"))
    };

    let mut handle = if let Some(output) = output {
        let (mut reader, writer) = os_pipe::pipe().with_context(|| {
            format!("Failed to create a pipe to run the command `{description}``")
        })?;

        let writer_clone = writer.try_clone().with_context(|| {
            format!("Failed to clone the pipe writer for the command `{description}`")
        })?;

        cmd.stdout(writer_clone).stderr(writer);
        let handle = spawn(cmd)?;

        reader
            .read_to_end(output)
            .with_context(|| format!("Failed to read the output of the command `{description}`"))?;

        output.push(b'\n');

        handle
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
        spawn(cmd)?
    };

    handle
        .wait()
        .with_context(|| format!("Failed to wait on the command `{description}` to exit"))
        .map(|status| status.success())
}

// Parses parts of the output of `cargo metadata`.
#[derive(Deserialize)]
struct CargoMetadata {
    target_directory: PathBuf,
}

pub struct CmdRunner {
    target_dir: PathBuf,
}

impl CmdRunner {
    pub fn build() -> Result<Self> {
        // Get the target directory from Cargo.
        let metadata_output = Command::new("cargo")
            .arg("metadata")
            .arg("-q")
            .arg("--format-version")
            .arg("1")
            .arg("--no-deps")
            .stdin(Stdio::null())
            .stderr(Stdio::inherit())
            .output()
            .context(CARGO_METADATA_ERR)?;

        if !metadata_output.status.success() {
            bail!("The command `cargo metadata …` failed. Are you in the `rustlings/` directory?");
        }

        let metadata: CargoMetadata = serde_json::de::from_slice(&metadata_output.stdout)
            .context(
                "Failed to read the field `target_directory` from the output of the command `cargo metadata …`",
            )?;

        Ok(Self {
            target_dir: metadata.target_directory,
        })
    }

    pub fn cargo<'out>(
        &self,
        subcommand: &str,
        bin_name: &str,
        output: Option<&'out mut Vec<u8>>,
    ) -> CargoSubcommand<'out> {
        let mut cmd = Command::new("cargo");
        cmd.arg(subcommand).arg("-q").arg("--bin").arg(bin_name);

        // A hack to make `cargo run` work when developing Rustlings.
        #[cfg(debug_assertions)]
        cmd.arg("--manifest-path")
            .arg("dev/Cargo.toml")
            .arg("--target-dir")
            .arg(&self.target_dir);

        if output.is_some() {
            cmd.arg("--color").arg("always");
        }

        CargoSubcommand { cmd, output }
    }

    /// The boolean in the returned `Result` is true if the command's exit status is success.
    pub fn run_debug_bin(&self, bin_name: &str, output: Option<&mut Vec<u8>>) -> Result<bool> {
        // 7 = "/debug/".len()
        let mut bin_path =
            PathBuf::with_capacity(self.target_dir.as_os_str().len() + 7 + bin_name.len());
        bin_path.push(&self.target_dir);
        bin_path.push("debug");
        bin_path.push(bin_name);

        run_cmd(Command::new(&bin_path), &bin_path.to_string_lossy(), output)
    }
}

pub struct CargoSubcommand<'out> {
    cmd: Command,
    output: Option<&'out mut Vec<u8>>,
}

impl<'out> CargoSubcommand<'out> {
    #[inline]
    pub fn args<'arg, I>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = &'arg str>,
    {
        self.cmd.args(args);
        self
    }

    /// The boolean in the returned `Result` is true if the command's exit status is success.
    #[inline]
    pub fn run(self, description: &str) -> Result<bool> {
        run_cmd(self.cmd, description, self.output)
    }
}

const CARGO_METADATA_ERR: &str = "Failed to run the command `cargo metadata …`
Did you already install Rust?
Try running `cargo --version` to diagnose the problem.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_cmd() {
        let mut cmd = Command::new("echo");
        cmd.arg("Hello");

        let mut output = Vec::with_capacity(8);
        run_cmd(cmd, "echo …", Some(&mut output)).unwrap();

        assert_eq!(output, b"Hello\n\n");
    }
}
