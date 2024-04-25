use anyhow::{Context, Result};
use crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    io::{Read, Write},
    process::{Command, Stdio},
};

use crate::{in_official_repo, terminal_link::TerminalFileLink, DEBUG_PROFILE};

pub const OUTPUT_CAPACITY: usize = 1 << 14;

fn run_command(
    mut cmd: Command,
    cmd_description: &str,
    output: &mut Vec<u8>,
    stderr: bool,
) -> Result<bool> {
    let (mut reader, writer) = os_pipe::pipe().with_context(|| {
        format!("Failed to create a pipe to run the command `{cmd_description}``")
    })?;

    let (stdout, stderr) = if stderr {
        (
            Stdio::from(writer.try_clone().with_context(|| {
                format!("Failed to clone the pipe writer for the command `{cmd_description}`")
            })?),
            Stdio::from(writer),
        )
    } else {
        (Stdio::from(writer), Stdio::null())
    };

    let mut handle = cmd
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .with_context(|| format!("Failed to run the command `{cmd_description}`"))?;

    // Prevent pipe deadlock.
    drop(cmd);

    reader
        .read_to_end(output)
        .with_context(|| format!("Failed to read the output of the command `{cmd_description}`"))?;

    output.push(b'\n');

    handle
        .wait()
        .with_context(|| format!("Failed to wait on the command `{cmd_description}` to exit"))
        .map(|status| status.success())
}

pub struct Exercise {
    pub dir: Option<&'static str>,
    // Exercise's unique name
    pub name: &'static str,
    // Exercise's path
    pub path: &'static str,
    pub test: bool,
    pub strict_clippy: bool,
    // The hint text associated with the exercise
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    fn run_bin(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Output".underlined())?;

        let bin_path = format!("target/debug/{}", self.name);
        let success = run_command(Command::new(&bin_path), &bin_path, output, true)?;

        if !success {
            writeln!(
                output,
                "{}",
                "The exercise didn't run successfully (nonzero exit code)"
                    .bold()
                    .red()
            )?;
        }

        Ok(success)
    }

    fn cargo_cmd(
        &self,
        command: &str,
        args: &[&str],
        cmd_description: &str,
        output: &mut Vec<u8>,
        dev: bool,
        stderr: bool,
    ) -> Result<bool> {
        let mut cmd = Command::new("cargo");
        cmd.arg(command);

        // A hack to make `cargo run` work when developing Rustlings.
        if dev {
            cmd.arg("--manifest-path")
                .arg("dev/Cargo.toml")
                .arg("--target-dir")
                .arg("target");
        }

        cmd.arg("--color")
            .arg("always")
            .arg("-q")
            .arg("--bin")
            .arg(self.name)
            .args(args);

        run_command(cmd, cmd_description, output, stderr)
    }

    pub fn run(&self, output: &mut Vec<u8>) -> Result<bool> {
        output.clear();

        // Developing the official Rustlings.
        let dev = DEBUG_PROFILE && in_official_repo();

        let build_success = self.cargo_cmd("build", &[], "cargo build …", output, dev, true)?;
        if !build_success {
            return Ok(false);
        }

        // Discard the output of `cargo build` because it will be shown again by the Cargo command.
        output.clear();

        let clippy_args: &[&str] = if self.strict_clippy {
            &["--profile", "test", "--", "-D", "warnings"]
        } else {
            &["--profile", "test"]
        };
        let clippy_success =
            self.cargo_cmd("clippy", clippy_args, "cargo clippy …", output, dev, true)?;
        if !clippy_success {
            return Ok(false);
        }

        if !self.test {
            return self.run_bin(output);
        }

        let test_success = self.cargo_cmd(
            "test",
            &[
                "--",
                "--color",
                "always",
                "--nocapture",
                "--format",
                "pretty",
            ],
            "cargo test …",
            output,
            dev,
            // Hide warnings because they are shown by Clippy.
            false,
        )?;

        let run_success = self.run_bin(output)?;

        Ok(test_success && run_success)
    }

    pub fn terminal_link(&self) -> StyledContent<TerminalFileLink<'_>> {
        style(TerminalFileLink(self.path)).underlined().blue()
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}
