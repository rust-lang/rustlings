use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{path::Path, process::exit};

mod consts;
mod embedded;
mod exercise;
mod init;
mod list;
mod progress_bar;
mod run;
mod state_file;
mod verify;
mod watch;

use self::{
    consts::WELCOME,
    exercise::{Exercise, InfoFile},
    run::run,
    state_file::StateFile,
    verify::{verify, VerifyState},
};

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

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> Result<(usize, &'a Exercise)> {
    if name == "next" {
        for (ind, exercise) in exercises.iter().enumerate() {
            if !exercise.looks_done()? {
                return Ok((ind, exercise));
            }
        }

        println!("🎉 Congratulations! You have done all the exercises!");
        println!("🔚 There are no more exercises to do next!");
        exit(0);
    }

    exercises
        .iter()
        .enumerate()
        .find(|(_, exercise)| exercise.name == name)
        .with_context(|| format!("No exercise found for '{name}'!"))
}

fn main() -> Result<()> {
    let args = Args::parse();

    which::which("cargo").context(
        "Failed to find `cargo`.
Did you already install Rust?
Try running `cargo --version` to diagnose the problem.",
    )?;

    let exercises = InfoFile::parse()?.exercises;

    if matches!(args.command, Some(Subcommands::Init)) {
        init::init(&exercises).context("Initialization failed")?;
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

    let mut state_file = StateFile::read_or_default(&exercises);

    match args.command {
        None | Some(Subcommands::Watch) => {
            watch::watch(&state_file, &exercises)?;
        }
        // `Init` is handled above.
        Some(Subcommands::Init) => (),
        Some(Subcommands::List) => {
            list::list(&mut state_file, &exercises)?;
        }
        Some(Subcommands::Run { name }) => {
            let (_, exercise) = find_exercise(&name, &exercises)?;
            run(exercise).unwrap_or_else(|_| exit(1));
        }
        Some(Subcommands::Reset { name }) => {
            let (ind, exercise) = find_exercise(&name, &exercises)?;
            exercise.reset()?;
            state_file.reset(ind)?;
            println!("The exercise {exercise} has been reset!");
        }
        Some(Subcommands::Hint { name }) => {
            let (_, exercise) = find_exercise(&name, &exercises)?;
            println!("{}", exercise.hint);
        }
        Some(Subcommands::Verify) => match verify(&exercises, 0)? {
            VerifyState::AllExercisesDone => println!("All exercises done!"),
            VerifyState::Failed(exercise) => bail!("Exercise {exercise} failed"),
        },
    }

    Ok(())
}
