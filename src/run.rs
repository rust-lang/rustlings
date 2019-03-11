use crate::util::clean;
use crate::verify::test;
use console::{style, Emoji};
use indicatif::ProgressBar;
use std::fs;
use std::process::Command;
use toml::Value;

pub fn run(matches: clap::ArgMatches) -> Result<(), ()> {
    if let Some(filename) = matches.value_of("file") {
        let toml: Value = fs::read_to_string("info.toml").unwrap().parse().unwrap();
        let tomlvec: &Vec<Value> = toml.get("exercises").unwrap().as_array().unwrap();
        let mut exercises = tomlvec.clone();
        exercises.retain(|i| i.get("path").unwrap().as_str().unwrap() == filename);
        if exercises.is_empty() {
            println!("No exercise found for your filename!");
            std::process::exit(1);
        }

        let exercise: &Value = &exercises[0];
        match exercise.get("mode").unwrap().as_str().unwrap() {
            "test" => test(exercise.get("path").unwrap().as_str().unwrap())?,
            "compile" => compile_and_run(exercise.get("path").unwrap().as_str().unwrap())?,
            _ => (),
        }
        Ok(())
    } else {
        panic!("Please supply a filename!");
    }
}

pub fn compile_and_run(filename: &str) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", filename).as_str());
    progress_bar.enable_steady_tick(100);
    let compilecmd = Command::new("rustc")
        .args(&[filename, "-o", "temp", "--color", "always"])
        .output()
        .expect("fail");
    progress_bar.set_message(format!("Running {}...", filename).as_str());
    if compilecmd.status.success() {
        let runcmd = Command::new("./temp").output().expect("fail");
        progress_bar.finish_and_clear();

        if runcmd.status.success() {
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            let formatstr = format!("{} Successfully ran {}", Emoji("✅", "✓"), filename);
            println!("{}", style(formatstr).green());
            clean();
            Ok(())
        } else {
            println!("{}", String::from_utf8_lossy(&runcmd.stdout));
            println!("{}", String::from_utf8_lossy(&runcmd.stderr));

            let formatstr = format!("{} Ran {} with errors", Emoji("⚠️ ", "!"), filename);
            println!("{}", style(formatstr).red());
            clean();
            Err(())
        }
    } else {
        progress_bar.finish_and_clear();
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
        clean();
        Err(())
    }
}
