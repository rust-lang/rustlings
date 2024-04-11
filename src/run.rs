use anyhow::{bail, Result};
use crossterm::style::Stylize;
use std::io::{stdout, Write};

use crate::app_state::{AppState, ExercisesProgress};

pub fn run(app_state: &mut AppState) -> Result<()> {
    let exercise = app_state.current_exercise();
    let output = exercise.run()?;

    {
        let mut stdout = stdout().lock();
        stdout.write_all(&output.stdout)?;
        stdout.write_all(&output.stderr)?;
        stdout.flush()?;
    }

    if !output.status.success() {
        bail!("Ran {exercise} with errors");
    }

    println!(
        "{}{}",
        "✓ Successfully ran ".green(),
        exercise.path.to_string_lossy().green(),
    );

    match app_state.done_current_exercise()? {
        ExercisesProgress::AllDone => println!(
            "🎉 Congratulations! You have done all the exercises!
🔚 There are no more exercises to do next!"
        ),
        ExercisesProgress::Pending => println!("Next exercise: {}", app_state.current_exercise()),
    }

    Ok(())
}
