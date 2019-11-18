use crate::exercise::{Exercise, Mode, State};
use console::{style, Emoji};
use indicatif::ProgressBar;

pub fn verify<'a>(start_at: impl IntoIterator<Item = &'a Exercise>) -> Result<(), &'a Exercise> {
    for exercise in start_at {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test_interactively(&exercise),
            Mode::Compile => compile_only(&exercise),
        };
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
    }
    Ok(())
}

pub fn test(exercise: &Exercise) -> Result<(), ()> {
    compile_and_test(exercise, true)?;
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
        Ok(prompt_for_completion(&exercise))
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

fn compile_and_test_interactively(exercise: &Exercise) -> Result<bool, ()> {
    compile_and_test(exercise, false)
}

fn compile_and_test(exercise: &Exercise, skip_prompt: bool) -> Result<bool, ()> {
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
            Ok(skip_prompt || prompt_for_completion(exercise))
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

fn prompt_for_completion(exercise: &Exercise) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };

    let success_msg = match exercise.mode {
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

    false
}
