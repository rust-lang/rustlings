use anyhow::Result;
use crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    cmd::{run_cmd, CargoCmd},
    in_official_repo,
    terminal_link::TerminalFileLink,
    DEBUG_PROFILE,
};

pub const OUTPUT_CAPACITY: usize = 1 << 14;

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
    fn run_bin(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        writeln!(output, "{}", "Output".underlined())?;

        let mut bin_path =
            PathBuf::with_capacity(target_dir.as_os_str().len() + 7 + self.name.len());
        bin_path.push(target_dir);
        bin_path.push("debug");
        bin_path.push(self.name);

        let success = run_cmd(Command::new(&bin_path), &bin_path.to_string_lossy(), output)?;

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

    pub fn run(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        output.clear();

        // Developing the official Rustlings.
        let dev = DEBUG_PROFILE && in_official_repo();

        let build_success = CargoCmd {
            subcommand: "build",
            args: &[],
            exercise_name: self.name,
            description: "cargo build …",
            hide_warnings: false,
            target_dir,
            output,
            dev,
        }
        .run()?;
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
        let clippy_success = CargoCmd {
            subcommand: "clippy",
            args: clippy_args,
            exercise_name: self.name,
            description: "cargo clippy …",
            hide_warnings: false,
            target_dir,
            output,
            dev,
        }
        .run()?;
        if !clippy_success {
            return Ok(false);
        }

        if !self.test {
            return self.run_bin(output, target_dir);
        }

        let test_success = CargoCmd {
            subcommand: "test",
            args: &[
                "--",
                "--color",
                "always",
                "--show-output",
                "--format",
                "pretty",
            ],
            exercise_name: self.name,
            description: "cargo test …",
            // Hide warnings because they are shown by Clippy.
            hide_warnings: true,
            target_dir,
            output,
            dev,
        }
        .run()?;

        let run_success = self.run_bin(output, target_dir)?;

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
