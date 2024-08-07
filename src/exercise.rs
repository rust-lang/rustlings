use anyhow::Result;
use ratatui::crossterm::style::{style, StyledContent, Stylize};
use std::{
    fmt::{self, Display, Formatter},
    io::Write,
};

use crate::{cmd::CmdRunner, terminal_link::TerminalFileLink};

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

// Run an exercise binary and append its output to the `output` buffer.
// Compilation must be done before calling this method.
fn run_bin(
    bin_name: &str,
    mut output: Option<&mut Vec<u8>>,
    cmd_runner: &CmdRunner,
) -> Result<bool> {
    if let Some(output) = output.as_deref_mut() {
        writeln!(output, "{}", "Output".underlined())?;
    }

    let success = cmd_runner.run_debug_bin(bin_name, output.as_deref_mut())?;

    if let Some(output) = output {
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
    pub hint: &'static str,
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
    fn run(
        &self,
        bin_name: &str,
        mut output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        let build_success = cmd_runner
            .cargo("build", bin_name, output.as_deref_mut())
            .run("cargo build …")?;
        if !build_success {
            return Ok(false);
        }

        // Discard the compiler output because it will be shown again by `cargo test` or Clippy.
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }

        if self.test() {
            let output_is_some = output.is_some();
            let mut test_cmd = cmd_runner.cargo("test", bin_name, output.as_deref_mut());
            if output_is_some {
                test_cmd.args(["--", "--color", "always", "--show-output"]);
            }
            let test_success = test_cmd.run("cargo test …")?;
            if !test_success {
                run_bin(bin_name, output, cmd_runner)?;
                return Ok(false);
            }

            // Discard the compiler output because it will be shown again by Clippy.
            if let Some(output) = output.as_deref_mut() {
                output.clear();
            }
        }

        let mut clippy_cmd = cmd_runner.cargo("clippy", bin_name, output.as_deref_mut());

        // `--profile test` is required to also check code with `[cfg(test)]`.
        if self.strict_clippy() {
            clippy_cmd.args(["--profile", "test", "--", "-D", "warnings"]);
        } else {
            clippy_cmd.args(["--profile", "test"]);
        }

        let clippy_success = clippy_cmd.run("cargo clippy …")?;
        let run_success = run_bin(bin_name, output, cmd_runner)?;

        Ok(clippy_success && run_success)
    }

    /// Compile, check and run the exercise.
    /// The output is written to the `output` buffer after clearing it.
    #[inline]
    fn run_exercise(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        self.run(self.name(), output, cmd_runner)
    }

    /// Compile, check and run the exercise's solution.
    /// The output is written to the `output` buffer after clearing it.
    fn run_solution(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len() + 4);
        bin_name.push_str(name);
        bin_name.push_str("_sol");

        self.run(&bin_name, output, cmd_runner)
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
