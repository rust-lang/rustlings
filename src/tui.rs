use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::stdout,
    path::Path,
    sync::mpsc::{channel, RecvTimeoutError},
    time::Duration,
};

use crate::{
    exercise::Exercise,
    verify::{verify, VerifyState},
};

fn watch(exercises: &[Exercise]) -> Result<()> {
    let (tx, rx) = channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    let mut failed_exercise_hint = match verify(exercises, (0, exercises.len()))? {
        VerifyState::AllExercisesDone => return Ok(()),
        VerifyState::Failed(exercise) => Some(&exercise.hint),
    };

    let mut pending_exercises = Vec::with_capacity(exercises.len());
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                Ok(events) => {
                    for event in events {
                        if event.kind == DebouncedEventKind::Any
                            && event.path.extension().is_some_and(|ext| ext == "rs")
                        {
                            pending_exercises.extend(exercises.iter().filter(|exercise| {
                                !exercise.looks_done().unwrap_or(false)
                                    || event.path.ends_with(&exercise.path)
                            }));
                            let num_done = exercises.len() - pending_exercises.len();

                            match verify(
                                pending_exercises.iter().copied(),
                                (num_done, exercises.len()),
                            )? {
                                VerifyState::AllExercisesDone => return Ok(()),
                                VerifyState::Failed(exercise) => {
                                    failed_exercise_hint = Some(&exercise.hint);
                                }
                            }

                            pending_exercises.clear();
                        }
                    }
                }
                Err(e) => println!("watch error: {e:?}"),
            },
            Err(RecvTimeoutError::Timeout) => {
                // the timeout expired, just check the `should_quit` variable below then loop again
            }
            Err(e) => println!("watch error: {e:?}"),
        }

        // TODO: Check if we need to exit
    }
}

pub fn tui(exercises: &[Exercise]) -> Result<()> {
    let mut stdout = stdout().lock();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(&mut stdout))?;
    terminal.clear()?;

    watch(exercises)?;

    drop(terminal);
    stdout.execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    // TODO
    println!("We hope you're enjoying learning about Rust!");
    println!("If you want to continue working on the exercises at a later point, you can simply run `rustlings watch` again");

    Ok(())
}
