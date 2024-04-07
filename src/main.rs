use crate::consts::WELCOME;
use crate::embedded::{WriteStrategy, EMBEDDED_FILES};
use crate::exercise::{Exercise, ExerciseList};
use crate::run::run;
use crate::verify::verify;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use state::State;
use std::path::Path;
use std::process::exit;
use verify::VerifyState;

mod consts;
mod embedded;
mod exercise;
mod init;
mod list;
mod run;
mod state;
mod verify;
mod watch;

/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
#[derive(Parser)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Initialize Rustlings
    Init,
    /// Verify all exercises according to the recommended order
    Verify,
    /// Same as just running `rustlings` without a subcommand.
    Watch,
    /// Run/Test a single exercise
    Run {
        /// The name of the exercise
        name: String,
    },
    /// Reset a single exercise
    Reset {
        /// The name of the exercise
        name: String,
    },
    /// Return a hint for the given exercise
    Hint {
        /// The name of the exercise
        name: String,
    },
    /// List the exercises available in Rustlings
    List,
}

fn main() -> Result<()> {
    let args = Args::parse();

    which::which("cargo").context(
        "Failed to find `cargo`.
Did you already install Rust?
Try running `cargo --version` to diagnose the problem.",
    )?;

    let exercises = ExerciseList::parse()?.exercises;

    if matches!(args.command, Some(Subcommands::Init)) {
        init::init_rustlings(&exercises).context("Initialization failed")?;
        println!(
            "\nDone initialization!\n
Run `cd rustlings` to go into the generated directory.
Then run `rustlings` for further instructions on getting started."
        );
        return Ok(());
    } else if !Path::new("exercises").is_dir() {
        println!(
            "
{WELCOME}

The `exercises` directory wasn't found in the current directory.
If you are just starting with Rustlings, run the command `rustlings init` to initialize it."
        );
        exit(1);
    }

    let state = State::read_or_default(&exercises);

    match args.command {
        None | Some(Subcommands::Watch) => {
            watch::watch(&state, &exercises)?;
        }
        // `Init` is handled above.
        Some(Subcommands::Init) => (),
        Some(Subcommands::List) => {
            list::list(&state, &exercises)?;
        }
        Some(Subcommands::Run { name }) => {
            let exercise = find_exercise(&name, &exercises)?;
            run(exercise).unwrap_or_else(|_| exit(1));
        }
        Some(Subcommands::Reset { name }) => {
            let exercise = find_exercise(&name, &exercises)?;
            EMBEDDED_FILES
                .write_exercise_to_disk(&exercise.path, WriteStrategy::Overwrite)
                .with_context(|| format!("Failed to reset the exercise {exercise}"))?;
            println!("The file {} has been reset!", exercise.path.display());
        }
        Some(Subcommands::Hint { name }) => {
            let exercise = find_exercise(&name, &exercises)?;
            println!("{}", exercise.hint);
        }
        Some(Subcommands::Verify) => match verify(&exercises, 0)? {
            VerifyState::AllExercisesDone => println!("All exercises done!"),
            VerifyState::Failed(exercise) => bail!("Exercise {exercise} failed"),
        },
    }

    Ok(())
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> Result<&'a Exercise> {
    if name == "next" {
        for exercise in exercises {
            if !exercise.looks_done()? {
                return Ok(exercise);
            }
        }

        println!("🎉 Congratulations! You have done all the exercises!");
        println!("🔚 There are no more exercises to do next!");
        exit(0);
    }

    exercises
        .iter()
        .find(|e| e.name == name)
        .with_context(|| format!("No exercise found for '{name}'!"))
}
