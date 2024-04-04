use anyhow::Result;
use console::style;
use std::io::{stdout, Write};

use crate::exercise::{Exercise, Mode, State};

pub enum VerifyState<'a> {
    AllExercisesDone,
    Failed(&'a Exercise),
}

// Verify that the provided container of Exercise objects
// can be compiled and run without any failures.
// Any such failures will be reported to the end user.
// If the Exercise being verified is a test, the verbose boolean
// determines whether or not the test harness outputs are displayed.
pub fn verify<'a>(
    pending_exercises: impl IntoIterator<Item = &'a Exercise>,
    progress: (usize, usize),
) -> Result<VerifyState<'a>> {
    let (mut num_done, total) = progress;
    println!(
        "Progress: {num_done}/{total} ({:.1}%)\n",
        num_done as f32 / total as f32 * 100.0,
    );

    for exercise in pending_exercises {
        let output = exercise.run()?;

        {
            let mut stdout = stdout().lock();
            stdout.write_all(&output.stdout)?;
            stdout.write_all(&output.stderr)?;
            stdout.flush()?;
        }

        if !output.status.success() {
            return Ok(VerifyState::Failed(exercise));
        }

        println!();
        match exercise.mode {
            Mode::Compile => success!("Successfully ran {}!", exercise),
            Mode::Test => success!("Successfully tested {}!", exercise),
            Mode::Clippy => success!("Successfully checked {}!", exercise),
        }

        if let State::Pending(context) = exercise.state()? {
            println!(
                "\nYou can keep working on this exercise,
or jump into the next one by removing the {} comment:\n",
                style("`I AM NOT DONE`").bold()
            );

            for context_line in context {
                let formatted_line = if context_line.important {
                    format!("{}", style(context_line.line).bold())
                } else {
                    context_line.line
                };

                println!(
                    "{:>2} {}  {}",
                    style(context_line.number).blue().bold(),
                    style("|").blue(),
                    formatted_line,
                );
            }
            return Ok(VerifyState::Failed(exercise));
        }

        num_done += 1;
        println!(
            "Progress: {num_done}/{total} ({:.1}%)\n",
            num_done as f32 / total as f32 * 100.0,
        );
    }

    Ok(VerifyState::AllExercisesDone)
}
