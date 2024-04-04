use anyhow::{bail, Result};
use std::io::{stdout, Write};
use std::time::Duration;

use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

// Invoke the rust compiler on the path of the given exercise,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the exercise is test)
pub fn run(exercise: &Exercise, verbose: bool) -> Result<()> {
    match exercise.mode {
        Mode::Test => test(exercise, verbose),
        Mode::Compile | Mode::Clippy => compile_and_run(exercise),
    }
}

// Compile and run an exercise.
// This is strictly for non-test binaries, so output is displayed
fn compile_and_run(exercise: &Exercise) -> Result<()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Running {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let output = exercise.run()?;
    progress_bar.finish_and_clear();

    stdout().write_all(&output.stdout)?;
    if !output.status.success() {
        stdout().write_all(&output.stderr)?;
        warn!("Ran {} with errors", exercise);
        bail!("TODO");
    }

    success!("Successfully ran {}", exercise);
    Ok(())
}
