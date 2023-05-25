use std::process::Command;

use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

// Invoke the rust compiler on the path of the given exercise,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the exercise is test)
pub fn run(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => test(exercise, verbose)?,
        Mode::Compile => compile_and_run(exercise)?,
        Mode::Clippy => compile_and_run(exercise)?,
    }
    Ok(())
}

// Resets the exercise by stashing the changes.
pub fn reset(exercise: &Exercise) -> Result<(), ()> {
    let command = Command::new("git")
        .args(["stash", "--"])
        .arg(&exercise.path)
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

// Invoke the rust compiler on the path of the given exercise
// and run the ensuing binary.
// This is strictly for non-test binaries, so output is displayed
fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {exercise}..."));
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compilation of {} failed!, Compiler error message:\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("Running {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("Successfully ran {}", exercise);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("Ran {} with errors", exercise);
            Err(())
        }
    }
}
