use anyhow::Result;
use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    QueueableCommand,
};
use std::{
    io::{self, Write},
    process::exit,
};

use crate::{
    app_state::{AppState, ExercisesProgress},
    exercise::{solution_link_line, RunnableExercise, OUTPUT_CAPACITY},
};

pub fn run(app_state: &mut AppState) -> Result<()> {
    let exercise = app_state.current_exercise();
    let mut output = Vec::with_capacity(OUTPUT_CAPACITY);
    let success = exercise.run_exercise(Some(&mut output), app_state.cmd_runner())?;

    let mut stdout = io::stdout().lock();
    stdout.write_all(&output)?;

    if !success {
        app_state.set_pending(app_state.current_exercise_ind())?;

        stdout.write_all(b"Ran ")?;
        app_state
            .current_exercise()
            .terminal_file_link(&mut stdout)?;
        stdout.write_all(b" with errors\n")?;
        exit(1);
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("✓ Successfully ran ".as_bytes())?;
    stdout.write_all(exercise.path.as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n")?;

    if let Some(solution_path) = app_state.current_solution_path()? {
        stdout.write_all(b"\n")?;
        solution_link_line(&mut stdout, &solution_path)?;
        stdout.write_all(b"\n")?;
    }

    match app_state.done_current_exercise(&mut stdout)? {
        ExercisesProgress::NewPending | ExercisesProgress::CurrentPending => {
            stdout.write_all(b"Next exercise: ")?;
            app_state
                .current_exercise()
                .terminal_file_link(&mut stdout)?;
            stdout.write_all(b"\n")?;
        }
        ExercisesProgress::AllDone => (),
    }

    Ok(())
}
