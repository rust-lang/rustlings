use crate::exercise::{Exercise, Mode, State};
use console::style;
use indicatif::ProgressBar;

pub fn verify<'a>(start_at: impl IntoIterator<Item = &'a Exercise>) -> Result<(), &'a Exercise> {
    for exercise in start_at {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test(&exercise, RunMode::Interactive),
            Mode::Compile => compile_only(&exercise),
            Mode::Clippy => compile_only(&exercise),
        };
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
    }
    Ok(())
}

enum RunMode {
    Interactive,
    NonInteractive,
}

pub fn test(exercise: &Exercise) -> Result<(), ()> {
    compile_and_test(exercise, RunMode::NonInteractive)?;
    Ok(())
}

fn compile_only(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);
    let compilation_result = exercise.compile();
    progress_bar.finish_and_clear();

    match compilation_result {
        Ok(_) => {
            success!("Successfully compiled {}!", exercise);
            Ok(prompt_for_completion(&exercise))
        }
        Err(output) => {
            warn!(
                "Compilation of {} failed! Compiler error message:\n",
                exercise
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

fn compile_and_test(exercise: &Exercise, run_mode: RunMode) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();

    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compiling of {} failed! Please try again. Here's the output:",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(_) => {
            if let RunMode::Interactive = run_mode {
                Ok(prompt_for_completion(&exercise))
            } else {
                Ok(true)
            }
        }
        Err(output) => {
            warn!(
                "Testing of {} failed! Please try again. Here's the output:",
                exercise
            );
            println!("{}", output.stdout);
            Err(())
        }
    }
}

fn prompt_for_completion(exercise: &Exercise) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };

    let success_msg = match exercise.mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
        Mode::Clippy => "The code is compiling, and 📎 Clippy 📎 is happy!",
    };

    println!("");
    println!("🎉 🎉  {} 🎉 🎉", success_msg);
    println!("");
    println!("You can keep working on this exercise,");
    println!(
        "or jump into the next one by removing the {} comment:",
        style("`I AM NOT DONE`").bold()
    );
    println!("");
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            format!("{}", context_line.line)
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line
        );
    }

    false
}
