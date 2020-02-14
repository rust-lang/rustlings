use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

pub fn run(exercise: &Exercise) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => test(exercise)?,
        Mode::Compile => compile_and_run(exercise)?,
        Mode::Clippy => compile_and_run(exercise)?,
    }
    Ok(())
}

fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compilation of {} failed!, Compiler error message:\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("Running {}...", exercise).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("Successfully ran {}", exercise);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("Ran {} with errors", exercise);
            Err(())
        }
    }
}
