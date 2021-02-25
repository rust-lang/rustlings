use crate::exercise::{Exercise, ExerciseList};
use crate::run::run;
use crate::verify::verify;
use clap::{crate_version, App, Arg, SubCommand};
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod run;
mod verify;

fn main() {
    let matches = App::new("rustlings")
        .version(crate_version!())
        .author("Marisa, Carol Nichols")
        .about("Rustlings is a collection of small exercises to get you used to writing and reading Rust code")
        .arg(
            Arg::with_name("nocapture")
                .long("nocapture")
                .help("Show outputs from the test exercises")
        )
        .subcommand(
            SubCommand::with_name("verify")
                .alias("v")
                .about("Verifies all exercises according to the recommended order")
        )
        .subcommand(
            SubCommand::with_name("watch")
                .alias("w")
                .about("Reruns `verify` when files were edited")
        )
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .about("Runs/Tests a single exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("hint")
                .alias("h")
                .about("Returns a hint for the current exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("Lists the exercises available in rustlings")
                .arg(
                    Arg::with_name("paths")
                        .long("paths")
                        .short("p")
                        .conflicts_with("names")
                        .help("Show only the paths of the exercises")
                )
                .arg(
                    Arg::with_name("names")
                        .long("names")
                        .short("n")
                        .conflicts_with("paths")
                        .help("Show only the names of the exercises")
                )
                .arg(
                    Arg::with_name("filter")
                        .long("filter")
                        .short("f")
                        .takes_value(true)
                        .empty_values(false)
                        .help(
                            "Provide a string to match the exercise names.\
                            \nComma separated patterns are acceptable."
                        )
                )
                .arg(
                    Arg::with_name("unsolved")
                        .long("unsolved")
                        .short("u")
                        .conflicts_with("solved")
                        .help("Display only exercises not yet solved")
                )
                .arg(
                    Arg::with_name("solved")
                        .long("solved")
                        .short("s")
                        .conflicts_with("unsolved")
                        .help("Display only exercises that have been solved")
                )
        )
        .get_matches();

    if matches.subcommand_name().is_none() {
        println!();
        println!(r#"       welcome to...                      "#);
        println!(r#"                 _   _ _                  "#);
        println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
        println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
        println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
        println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
        println!(r#"                               |___/      "#);
        println!();
    }

    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the rustlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd rustlings/`!");
        std::process::exit(1);
    }

    if !rustc_exists() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let verbose = matches.is_present("nocapture");

    // Handle the list command
    if let Some(list_m) = matches.subcommand_matches("list") {
        if ["paths", "names"].iter().all(|k| !list_m.is_present(k)) {
            println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
        }
        let filters = list_m.value_of("filter").unwrap_or_default().to_lowercase();
        exercises.iter().for_each(|e| {
            let fname = format!("{}", e.path.display());
            let filter_cond = filters
                .split(',')
                .filter(|f| !f.trim().is_empty())
                .any(|f| e.name.contains(&f) || fname.contains(&f));
            let status = if e.looks_done() { "Done" } else { "Pending" };
            let solve_cond = {
                (e.looks_done() && list_m.is_present("solved"))
                    || (!e.looks_done() && list_m.is_present("unsolved"))
                    || (!list_m.is_present("solved") && !list_m.is_present("unsolved"))
            };
            if solve_cond && (filter_cond || !list_m.is_present("filter")) {
                let line = if list_m.is_present("paths") {
                    format!("{}\n", fname)
                } else if list_m.is_present("names") {
                    format!("{}\n", e.name)
                } else {
                    format!("{:<17}\t{:<46}\t{:<7}\n", e.name, fname, status)
                };
                // Somehow using println! leads to the binary panicking
                // when its output is piped.
                // So, we're handling a Broken Pipe error and exiting with 0 anyway
                let stdout = std::io::stdout();
                {
                    let mut handle = stdout.lock();
                    handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                        match e.kind() {
                            std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                            _ => std::process::exit(1),
                        };
                    });
                }
            }
        });
        std::process::exit(0);
    }

    // Handle the run command
    if let Some(ref matches) = matches.subcommand_matches("run") {
        let name = matches.value_of("name").unwrap();

        let matching_exercise = |e: &&Exercise| name == e.name;

        let exercise = exercises.iter().find(matching_exercise).unwrap_or_else(|| {
            println!("No exercise found for your given name!");
            std::process::exit(1)
        });

        run(&exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
    }

    if let Some(ref matches) = matches.subcommand_matches("hint") {
        let name = matches.value_of("name").unwrap();

        let exercise = exercises
            .iter()
            .find(|e| name == e.name)
            .unwrap_or_else(|| {
                println!("No exercise found for your given name!");
                std::process::exit(1)
            });

        println!("{}", exercise.hint);
    }

    // Handle the verify command
    if matches.subcommand_matches("verify").is_some() {
        verify(&exercises, verbose).unwrap_or_else(|_| std::process::exit(1));
    }

    // Handle the watch command
    if matches.subcommand_matches("watch").is_some() {
        if let Err(e) = watch(&exercises, verbose) {
            println!(
                "Error: Could not watch your progress. Error message was {:?}.",
                e
            );
            println!("Most likely you've run out of disk space or your 'inotify limit' has been reached.");
            std::process::exit(1);
        }
        println!(
            "{emoji} All exercises completed! {emoji}",
            emoji = Emoji("🎉", "★")
        );
        println!();
        println!("+----------------------------------------------------+");
        println!("|          You made it to the Fe-nish line!          |");
        println!("+--------------------------  ------------------------+");
        println!("                          \\/                         ");
        println!("     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒   ");
        println!("   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒ ");
        println!("   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒ ");
        println!(" ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒ ");
        println!("   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓ ");
        println!("     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒   ");
        println!("       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒     ");
        println!("         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒       ");
        println!("           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒         ");
        println!("             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒           ");
        println!("           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒         ");
        println!("         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒       ");
        println!("       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒     ");
        println!("       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒     ");
        println!("           ▒▒  ▒▒                      ▒▒  ▒▒         ");
        println!();
        println!("We hope you enjoyed learning about the various aspects of Rust!");
        println!("If you noticed any issues, please don't hesitate to report them to our repo.");
        println!("You can also contribute your own exercises to help the greater community!");
        println!();
        println!("Before reporting an issue or contributing, please read our guidelines:");
        println!("https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md");
    }

    if matches.subcommand_name().is_none() {
        let text = fs::read_to_string("default_out.txt").unwrap();
        println!("{}", text);
    }
}

fn spawn_watch_shell(failed_exercise_hint: &Arc<Mutex<Option<String>>>) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("Type 'hint' to get help or 'clear' to clear the screen");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.eq("hint") {
                    if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                        println!("{}", hint);
                    }
                } else if input.eq("clear") {
                    println!("\x1B[2J\x1B[1;1H");
                } else {
                    println!("unknown command: {}", input);
                }
            }
            Err(error) => println!("error reading command: {}", error),
        }
    });
}

fn watch(exercises: &[Exercise], verbose: bool) -> notify::Result<()> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let to_owned_hint = |t: &Exercise| t.hint.to_owned();
    let failed_exercise_hint = match verify(exercises.iter(), verbose) {
        Ok(_) => return Ok(()),
        Err(exercise) => Arc::new(Mutex::new(Some(to_owned_hint(exercise)))),
    };
    spawn_watch_shell(&failed_exercise_hint);
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .skip_while(|e| !filepath.ends_with(&e.path))
                            // .filter(|e| filepath.ends_with(&e.path))
                            .chain(
                                exercises
                                    .iter()
                                    .filter(|e| !e.looks_done() && !filepath.ends_with(&e.path))
                            );
                        clear_screen();
                        match verify(pending_exercises, verbose) {
                            Ok(_) => return Ok(()),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(to_owned_hint(exercise));
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn rustc_exists() -> bool {
    Command::new("rustc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
