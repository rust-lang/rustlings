use anyhow::Result;
use crossterm::style::{Attribute, ContentStyle, Stylize};
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
pub fn verify(exercises: &[Exercise], mut current_exercise_ind: usize) -> Result<VerifyState<'_>> {
    while current_exercise_ind < exercises.len() {
        let exercise = &exercises[current_exercise_ind];

        println!(
            "Progress: {current_exercise_ind}/{} ({:.1}%)\n",
            exercises.len(),
            current_exercise_ind as f32 / exercises.len() as f32 * 100.0,
        );

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
        // TODO: Color
        match exercise.mode {
            Mode::Compile => println!("Successfully ran {exercise}!"),
            Mode::Test => println!("Successfully tested {exercise}!"),
            Mode::Clippy => println!("Successfully checked {exercise}!"),
        }

        if let State::Pending(context) = exercise.state()? {
            println!(
                "\nYou can keep working on this exercise,
or jump into the next one by removing the {} comment:\n",
                "`I AM NOT DONE`".bold()
            );

            for context_line in context {
                let formatted_line = if context_line.important {
                    format!("{}", context_line.line.bold())
                } else {
                    context_line.line
                };

                println!(
                    "{:>2} {}  {}",
                    ContentStyle {
                        foreground_color: Some(crossterm::style::Color::Blue),
                        background_color: None,
                        underline_color: None,
                        attributes: Attribute::Bold.into()
                    }
                    .apply(context_line.number),
                    "|".blue(),
                    formatted_line,
                );
            }
            return Ok(VerifyState::Failed(exercise));
        }

        current_exercise_ind += 1;
    }

    Ok(VerifyState::AllExercisesDone)
}
