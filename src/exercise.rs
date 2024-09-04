use anyhow::Result;
use crossterm::{
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, StdoutLock, Write};

use crate::{
    cmd::CmdRunner,
    term::{self, terminal_file_link, write_ansi, CountedWrite},
};

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

pub fn solution_link_line(stdout: &mut StdoutLock, solution_path: &str) -> io::Result<()> {
    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(b"Solution")?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b" for comparison: ")?;
    if let Some(canonical_path) = term::canonicalize(solution_path) {
        terminal_file_link(stdout, solution_path, &canonical_path, Color::Cyan)?;
    } else {
        stdout.write_all(solution_path.as_bytes())?;
    }
    stdout.write_all(b"\n")
}

// Run an exercise binary and append its output to the `output` buffer.
// Compilation must be done before calling this method.
fn run_bin(
    bin_name: &str,
    mut output: Option<&mut Vec<u8>>,
    cmd_runner: &CmdRunner,
) -> Result<bool> {
    if let Some(output) = output.as_deref_mut() {
        write_ansi(output, SetAttribute(Attribute::Underlined));
        output.extend_from_slice(b"Output");
        write_ansi(output, ResetColor);
        output.push(b'\n');
    }

    let success = cmd_runner.run_debug_bin(bin_name, output.as_deref_mut())?;

    if let Some(output) = output {
        if !success {
            // This output is important to show the user that something went wrong.
            // Otherwise, calling something like `exit(1)` in an exercise without further output
            // leaves the user confused about why the exercise isn't done yet.
            write_ansi(output, SetAttribute(Attribute::Bold));
            write_ansi(output, SetForegroundColor(Color::Red));
            output.extend_from_slice(b"The exercise didn't run successfully (nonzero exit code)");
            write_ansi(output, ResetColor);
            output.push(b'\n');
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
    pub canonical_path: Option<String>,
    pub test: bool,
    pub strict_clippy: bool,
    pub hint: &'static str,
    pub done: bool,
}

impl Exercise {
    pub fn terminal_file_link<'a>(&self, writer: &mut impl CountedWrite<'a>) -> io::Result<()> {
        if let Some(canonical_path) = self.canonical_path.as_deref() {
            return terminal_file_link(writer, self.path, canonical_path, Color::Blue);
        }

        writer.write_str(self.path)
    }
}

pub trait RunnableExercise {
    fn name(&self) -> &str;
    fn dir(&self) -> Option<&str>;
    fn strict_clippy(&self) -> bool;
    fn test(&self) -> bool;

    // Compile, check and run the exercise or its solution (depending on `bin_name´).
    // The output is written to the `output` buffer after clearing it.
    fn run<const FORCE_STRICT_CLIPPY: bool>(
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
                test_cmd.args(["--", "--color", "always", "--format", "pretty"]);
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
        if FORCE_STRICT_CLIPPY || self.strict_clippy() {
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
        self.run::<false>(self.name(), output, cmd_runner)
    }

    /// Compile, check and run the exercise's solution.
    /// The output is written to the `output` buffer after clearing it.
    fn run_solution(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len() + 4);
        bin_name.push_str(name);
        bin_name.push_str("_sol");

        self.run::<true>(&bin_name, output, cmd_runner)
    }

    fn sol_path(&self) -> String {
        let name = self.name();

        let mut path = if let Some(dir) = self.dir() {
            // 14 = 10 + 1 + 3
            // solutions/ + / + .rs
            let mut path = String::with_capacity(14 + dir.len() + name.len());
            path.push_str("solutions/");
            path.push_str(dir);
            path.push('/');
            path
        } else {
            // 13 = 10 + 3
            // solutions/ + .rs
            let mut path = String::with_capacity(13 + name.len());
            path.push_str("solutions/");
            path
        };

        path.push_str(name);
        path.push_str(".rs");

        path
    }
}

impl RunnableExercise for Exercise {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn dir(&self) -> Option<&str> {
        self.dir
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
