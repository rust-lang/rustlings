use crate::exercise::{Exercise, Mode};
use console::{style, Emoji};
use indicatif::ProgressBar;

pub fn verify<'a>(start_at: impl IntoIterator<Item = &'a Exercise>) -> Result<(), ()> {
    for exercise in start_at {
        match exercise.mode {
            Mode::Test => test(&exercise)?,
            Mode::Compile => compile_only(&exercise)?,
        }
    }
    Ok(())
}

fn compile_only(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);
    let compile_output = exercise.compile();
    progress_bar.finish_and_clear();
    if compile_output.status.success() {
        let formatstr = format!(
            "{} Successfully compiled {}!",
            Emoji("✅", "✓"),
            exercise
        );
        println!("{}", style(formatstr).green());
        exercise.clean();
        Ok(())
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

pub fn test(exercise: &Exercise) -> Result<(), ()> {
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
            Ok(())
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
