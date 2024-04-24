use anyhow::{Context, Result};
use crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    io::{Read, Write},
    process::Command,
};

use crate::{in_official_repo, info_file::Mode, terminal_link::TerminalFileLink, DEBUG_PROFILE};

// TODO
pub const OUTPUT_CAPACITY: usize = 1 << 12;

fn run_command(mut cmd: Command, cmd_description: &str, output: &mut Vec<u8>) -> Result<bool> {
    let (mut reader, writer) = os_pipe::pipe().with_context(|| {
        format!("Failed to create a pipe to run the command `{cmd_description}``")
    })?;

    let mut handle = cmd
        .stdout(writer.try_clone().with_context(|| {
            format!("Failed to clone the pipe writer for the command `{cmd_description}`")
        })?)
        .stderr(writer)
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
    // The mode of the exercise
    pub mode: Mode,
    // The hint text associated with the exercise
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    fn run_bin(&self, output: &mut Vec<u8>) -> Result<bool> {
        writeln!(output, "{}", "Output".bold().magenta().underlined())?;

        let bin_path = format!("target/debug/{}", self.name);
        run_command(Command::new(&bin_path), &bin_path, output)
    }

    fn cargo_cmd(
        &self,
        command: &str,
        args: &[&str],
        cmd_description: &str,
        output: &mut Vec<u8>,
        dev: bool,
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

        run_command(cmd, cmd_description, output)
    }

    fn cargo_cmd_with_bin_output(
        &self,
        command: &str,
        args: &[&str],
        cmd_description: &str,
        output: &mut Vec<u8>,
        dev: bool,
    ) -> Result<bool> {
        // Discard the output of `cargo build` because it will be shown again by the Cargo command.
        output.clear();

        let cargo_cmd_success = self.cargo_cmd(command, args, cmd_description, output, dev)?;

        let run_success = self.run_bin(output)?;

        Ok(cargo_cmd_success && run_success)
    }

    pub fn run(&self, output: &mut Vec<u8>) -> Result<bool> {
        output.clear();

        // Developing the official Rustlings.
        let dev = DEBUG_PROFILE && in_official_repo();

        let build_success = self.cargo_cmd("build", &[], "cargo build …", output, dev)?;
        if !build_success {
            return Ok(false);
        }

        match self.mode {
            Mode::Run => self.run_bin(output),
            Mode::Test => self.cargo_cmd_with_bin_output(
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
            ),
            Mode::Clippy => self.cargo_cmd_with_bin_output(
                "clippy",
                &["--", "-D", "warnings"],
                "cargo clippy …",
                output,
                dev,
            ),
        }
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
