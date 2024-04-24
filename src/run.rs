use anyhow::{bail, Result};
use crossterm::style::{style, Stylize};
use std::io::{self, Write};

use crate::{
    app_state::{AppState, ExercisesProgress},
    terminal_link::TerminalFileLink,
};

pub fn run(app_state: &mut AppState) -> Result<()> {
    let exercise = app_state.current_exercise();
    let output = exercise.run()?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output.stdout)?;
    stdout.write_all(b"\n")?;
    stdout.write_all(&output.stderr)?;
    stdout.flush()?;

    if !output.status.success() {
        app_state.set_pending(app_state.current_exercise_ind())?;

        bail!(
            "Ran {} with errors",
            app_state.current_exercise().terminal_link(),
        );
    }

    stdout.write_fmt(format_args!(
        "{}{}\n",
        "✓ Successfully ran ".green(),
        exercise.path.green(),
    ))?;

    if let Some(solution_path) = app_state.current_solution_path()? {
        println!(
            "\nA solution file can be found at {}\n",
            style(TerminalFileLink(&solution_path)).underlined().green(),
        );
    }

    match app_state.done_current_exercise(&mut stdout)? {
        ExercisesProgress::AllDone => (),
        ExercisesProgress::Pending => println!(
            "Next exercise: {}",
            app_state.current_exercise().terminal_link(),
        ),
    }

    Ok(())
}
