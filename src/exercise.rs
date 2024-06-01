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

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

// Run an exercise binary and append its output to the `output` buffer.
// Compilation must be done before calling this method.
fn run_bin(bin_name: &str, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
    writeln!(output, "{}", "Output".underlined())?;

    // 7 = "/debug/".len()
    let mut bin_path = PathBuf::with_capacity(target_dir.as_os_str().len() + 7 + bin_name.len());
    bin_path.push(target_dir);
    bin_path.push("debug");
    bin_path.push(bin_name);

    let success = run_cmd(Command::new(&bin_path), &bin_path.to_string_lossy(), output)?;

    if !success {
        // This output is important to show the user that something went wrong.
        // Otherwise, calling something like `exit(1)` in an exercise without further output
        // leaves the user confused about why the exercise isn't done yet.
        writeln!(
            output,
            "{}",
            "The exercise didn't run successfully (nonzero exit code)"
                .bold()
                .red(),
        )?;
    }

    Ok(success)
}

/// See `info_file::ExerciseInfo`
pub struct Exercise {
    pub dir: Option<&'static str>,
    pub name: &'static str,
    /// Path of the exercise file starting with the `exercises/` directory.
    pub path: &'static str,
    pub test: bool,
    pub strict_clippy: bool,
    pub hint: String,
    pub done: bool,
}

impl Exercise {
    pub fn terminal_link(&self) -> StyledContent<TerminalFileLink<'_>> {
        style(TerminalFileLink(self.path)).underlined().blue()
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.path.fmt(f)
    }
}

pub trait RunnableExercise {
    fn name(&self) -> &str;
    fn strict_clippy(&self) -> bool;
    fn test(&self) -> bool;

    // Compile, check and run the exercise or its solution (depending on `bin_name´).
    // The output is written to the `output` buffer after clearing it.
    fn run(&self, bin_name: &str, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        output.clear();

        // Developing the official Rustlings.
        let dev = DEBUG_PROFILE && in_official_repo();

        let build_success = CargoCmd {
            subcommand: "build",
            args: &[],
            bin_name,
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

        // Discard the output of `cargo build` because it will be shown again by Clippy.
        output.clear();

        // `--profile test` is required to also check code with `[cfg(test)]`.
        let clippy_args: &[&str] = if self.strict_clippy() {
            &["--profile", "test", "--", "-D", "warnings"]
        } else {
            &["--profile", "test"]
        };
        let clippy_success = CargoCmd {
            subcommand: "clippy",
            args: clippy_args,
            bin_name,
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

        if !self.test() {
            return run_bin(bin_name, output, target_dir);
        }

        let test_success = CargoCmd {
            subcommand: "test",
            args: &["--", "--color", "always", "--show-output"],
            bin_name,
            description: "cargo test …",
            // Hide warnings because they are shown by Clippy.
            hide_warnings: true,
            target_dir,
            output,
            dev,
        }
        .run()?;

        let run_success = run_bin(bin_name, output, target_dir)?;

        Ok(test_success && run_success)
    }

    /// Compile, check and run the exercise.
    /// The output is written to the `output` buffer after clearing it.
    #[inline]
    fn run_exercise(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        self.run(self.name(), output, target_dir)
    }

    /// Compile, check and run the exercise's solution.
    /// The output is written to the `output` buffer after clearing it.
    fn run_solution(&self, output: &mut Vec<u8>, target_dir: &Path) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len());
        bin_name.push_str(name);
        bin_name.push_str("_sol");

        self.run(&bin_name, output, target_dir)
    }
}

impl RunnableExercise for Exercise {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn strict_clippy(&self) -> bool {
        self.strict_clippy
    }

    #[inline]
    fn test(&self) -> bool {
        self.test
    }
}
