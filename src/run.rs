use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use console::{style, Emoji};
use indicatif::ProgressBar;

pub fn run(exercise: &Exercise) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => test(exercise)?,
        Mode::Compile => compile_and_run(exercise)?,
    }
    Ok(())
}

pub fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilecmd = exercise.compile();
    progress_bar.set_message(format!("Running {}...", exercise).as_str());
    if compilecmd.status.success() {
        let runcmd = exercise.run();
        progress_bar.finish_and_clear();

        if runcmd.status.success() {
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            let formatstr = format!("{} Successfully ran {}", Emoji("✅", "✓"), exercise);
            println!("{}", style(formatstr).green());
            exercise.clean();
            Ok(())
        } else {
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            println!("{}", String::from_utf8_lossy(&runcmd.stderr));

            let formatstr = format!("{} Ran {} with errors", Emoji("⚠️ ", "!"), exercise);
            println!("{}", style(formatstr).red());
            exercise.clean();
            Err(())
        }
    } else {
        progress_bar.finish_and_clear();
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            exercise
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
        exercise.clean();
        Err(())
    }
}
