use crate::exercise::{ContextLine, Exercise, Mode, State};
use console::{style, Emoji};
use indicatif::ProgressBar;

pub fn verify<'a>(start_at: impl IntoIterator<Item = &'a Exercise>) -> Result<(), ()> {
    for exercise in start_at {
        let is_done = match exercise.mode {
            Mode::Test => test(&exercise)?,
            Mode::Compile => compile_only(&exercise)?,
        };
        if !is_done {
            return Err(());
        }
    }
    Ok(())
}

fn compile_only(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);
    let compile_output = exercise.compile();
    progress_bar.finish_and_clear();
    if compile_output.status.success() {
        let formatstr = format!("{} Successfully compiled {}!", Emoji("✅", "✓"), exercise);
        println!("{}", style(formatstr).green());
        exercise.clean();
        if let State::Pending(context) = exercise.state() {
            print_everything_looks_good(exercise.mode, context);
            Ok(false)
        } else {
            Ok(true)
        }
    } else {
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            exercise
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compile_output.stderr));
        exercise.clean();
        Err(())
    }
}

pub fn test(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compile_output = exercise.compile();
    if compile_output.status.success() {
        progress_bar.set_message(format!("Running {}...", exercise).as_str());

        let runcmd = exercise.run();
        progress_bar.finish_and_clear();

        if runcmd.status.success() {
            let formatstr = format!("{} Successfully tested {}!", Emoji("✅", "✓"), exercise);
            println!("{}", style(formatstr).green());
            exercise.clean();
            if let State::Pending(context) = exercise.state() {
                print_everything_looks_good(exercise.mode, context);
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            let formatstr = format!(
                "{} Testing of {} failed! Please try again. Here's the output:",
                Emoji("⚠️ ", "!"),
                exercise
            );
            println!("{}", style(formatstr).red());
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            exercise.clean();
            Err(())
        }
    } else {
        progress_bar.finish_and_clear();
        let formatstr = format!(
            "{} Compiling of {} failed! Please try again. Here's the output:",
            Emoji("⚠️ ", "!"),
            exercise
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compile_output.stderr));
        exercise.clean();
        Err(())
    }
}

fn print_everything_looks_good(mode: Mode, context: Vec<ContextLine>) {
    let success_msg = match mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
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
}
