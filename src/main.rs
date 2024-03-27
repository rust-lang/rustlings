use crate::exercise::{Exercise, ExerciseList};
use crate::project::write_project_json;
use crate::run::{reset, run};
use crate::verify::verify;
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::Emoji;
use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use shlex::Shlex;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;

/// Rustlings is a collection of small exercises to get you used to writing and reading Rust code
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Show outputs from the test exercises
    #[arg(long)]
    nocapture: bool,
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Verify all exercises according to the recommended order
    Verify,
    /// Rerun `verify` when files were edited
    Watch {
        /// Show hints on success
        #[arg(long)]
        success_hints: bool,
    },
    /// Run/Test a single exercise
    Run {
        /// The name of the exercise
        name: String,
    },
    /// Reset a single exercise using "git stash -- <filename>"
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
    List {
        /// Show only the paths of the exercises
        #[arg(short, long)]
        paths: bool,
        /// Show only the names of the exercises
        #[arg(short, long)]
        names: bool,
        /// Provide a string to match exercise names.
        /// Comma separated patterns are accepted
        #[arg(short, long)]
        filter: Option<String>,
        /// Display only exercises not yet solved
        #[arg(short, long)]
        unsolved: bool,
        /// Display only exercises that have been solved
        #[arg(short, long)]
        solved: bool,
    },
    /// Enable rust-analyzer for exercises
    Lsp,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.command.is_none() {
        println!("\n{WELCOME}\n");
    }

    if which::which("rustc").is_err() {
        println!("We cannot find `rustc`.");
        println!("Try running `rustc --version` to diagnose your problem.");
        println!("For instructions on how to install Rust, check the README.");
        std::process::exit(1);
    }

    let info_file = fs::read_to_string("info.toml").unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => println!(
                "The program must be run from the rustlings directory\nTry `cd rustlings/`!",
            ),
            _ => println!("Failed to read the info.toml file: {e}"),
        }
        std::process::exit(1);
    });
    let exercises = toml_edit::de::from_str::<ExerciseList>(&info_file)
        .unwrap()
        .exercises;
    let verbose = args.nocapture;

    let command = args.command.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });

    match command {
        Subcommands::List {
            paths,
            names,
            filter,
            unsolved,
            solved,
        } => {
            if !paths && !names {
                println!("{:<17}\t{:<46}\t{:<7}", "Name", "Path", "Status");
            }
            let mut exercises_done: u16 = 0;
            let lowercase_filter = filter
                .as_ref()
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            let filters = lowercase_filter
                .split(',')
                .filter_map(|f| {
                    let f = f.trim();
                    if f.is_empty() {
                        None
                    } else {
                        Some(f)
                    }
                })
                .collect::<Vec<_>>();

            for exercise in &exercises {
                let fname = exercise.path.to_string_lossy();
                let filter_cond = filters
                    .iter()
                    .any(|f| exercise.name.contains(f) || fname.contains(f));
                let looks_done = exercise.looks_done();
                let status = if looks_done {
                    exercises_done += 1;
                    "Done"
                } else {
                    "Pending"
                };
                let solve_cond =
                    (looks_done && solved) || (!looks_done && unsolved) || (!solved && !unsolved);
                if solve_cond && (filter_cond || filter.is_none()) {
                    let line = if paths {
                        format!("{fname}\n")
                    } else if names {
                        format!("{}\n", exercise.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", exercise.name)
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
            }

            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "Progress: You completed {} / {} exercises ({:.1} %).",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }

        Subcommands::Run { name } => {
            let exercise = find_exercise(&name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset { name } => {
            let exercise = find_exercise(&name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint { name } => {
            let exercise = find_exercise(&name, &exercises);

            println!("{}", exercise.hint);
        }

        Subcommands::Verify => {
            verify(&exercises, (0, exercises.len()), verbose, false)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Lsp => {
            if let Err(e) = write_project_json(exercises) {
                println!("Failed to write rust-project.json to disk for rust-analyzer: {e}");
            } else {
                println!("Successfully generated rust-project.json");
                println!("rust-analyzer will now parse exercises, restart your language server or editor");
            }
        }

        Subcommands::Watch { success_hints } => match watch(&exercises, verbose, success_hints) {
            Err(e) => {
                println!("Error: Could not watch your progress. Error message was {e:?}.");
                println!("Most likely you've run out of disk space or your 'inotify limit' has been reached.");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} All exercises completed! {emoji}",
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("We hope you're enjoying learning about Rust!");
                println!("If you want to continue working on the exercises at a later point, you can simply run `rustlings watch` again");
            }
        },
    }

    Ok(())
}

fn spawn_watch_shell(
    failed_exercise_hint: Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    println!("Welcome to watch mode! You can type 'help' to get an overview of the commands you can use here.");

    thread::spawn(move || {
        let mut input = String::with_capacity(32);
        let mut stdin = io::stdin().lock();

        loop {
            // Recycle input buffer.
            input.clear();

            if let Err(e) = stdin.read_line(&mut input) {
                println!("error reading command: {e}");
            }

            let input = input.trim();
            if input == "hint" {
                if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                    println!("{hint}");
                }
            } else if input == "clear" {
                println!("\x1B[2J\x1B[1;1H");
            } else if input == "quit" {
                should_quit.store(true, Ordering::SeqCst);
                println!("Bye!");
            } else if input == "help" {
                println!("{WATCH_MODE_HELP_MESSAGE}");
            } else if let Some(cmd) = input.strip_prefix('!') {
                let mut parts = Shlex::new(cmd);

                let Some(program) = parts.next() else {
                    println!("no command provided");
                    continue;
                };

                if let Err(e) = Command::new(program).args(parts).status() {
                    println!("failed to execute command `{cmd}`: {e}");
                }
            } else {
                println!("unknown command: {input}\n{WATCH_MODE_HELP_MESSAGE}");
            }
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name == "next" {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 Congratulations! You have done all the exercises!");
                println!("🔚 There are no more exercises to do next!");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("No exercise found for '{name}'!");
                std::process::exit(1)
            })
    }
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn watch(
    exercises: &[Exercise],
    verbose: bool,
    success_hints: bool,
) -> notify::Result<WatchStatus> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let failed_exercise_hint = match verify(
        exercises.iter(),
        (0, exercises.len()),
        verbose,
        success_hints,
    ) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => Arc::new(Mutex::new(Some(exercise.hint.clone()))),
    };
    spawn_watch_shell(Arc::clone(&failed_exercise_hint), Arc::clone(&should_quit));
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                Ok(events) => {
                    for event in events {
                        let event_path = event.path;
                        if event.kind == DebouncedEventKind::Any
                            && event_path.extension() == Some(OsStr::new("rs"))
                            && event_path.exists()
                        {
                            let filepath = event_path.as_path().canonicalize().unwrap();
                            let pending_exercises =
                                exercises
                                    .iter()
                                    .find(|e| filepath.ends_with(&e.path))
                                    .into_iter()
                                    .chain(exercises.iter().filter(|e| {
                                        !e.looks_done() && !filepath.ends_with(&e.path)
                                    }));
                            let num_done = exercises
                                .iter()
                                .filter(|e| e.looks_done() && !filepath.ends_with(&e.path))
                                .count();
                            clear_screen();
                            match verify(
                                pending_exercises,
                                (num_done, exercises.len()),
                                verbose,
                                success_hints,
                            ) {
                                Ok(_) => return Ok(WatchStatus::Finished),
                                Err(exercise) => {
                                    let mut failed_exercise_hint =
                                        failed_exercise_hint.lock().unwrap();
                                    *failed_exercise_hint = Some(exercise.hint.clone());
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("watch error: {e:?}"),
            },
            Err(RecvTimeoutError::Timeout) => {
                // the timeout expired, just check the `should_quit` variable below then loop again
            }
            Err(e) => println!("watch error: {e:?}"),
        }
        // Check if we need to exit
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

const DEFAULT_OUT: &str = "Thanks for installing Rustlings!

Is this your first time? Don't worry, Rustlings was made for beginners! We are
going to teach you a lot of things about Rust, but before we can get
started, here's a couple of notes about how Rustlings operates:

1. The central concept behind Rustlings is that you solve exercises. These
   exercises usually have some sort of syntax error in them, which will cause
   them to fail compilation or testing. Sometimes there's a logic error instead
   of a syntax error. No matter what error, it's your job to find it and fix it!
   You'll know when you fixed it because then, the exercise will compile and
   Rustlings will be able to move on to the next exercise.
2. If you run Rustlings in watch mode (which we recommend), it'll automatically
   start with the first exercise. Don't get confused by an error message popping
   up as soon as you run Rustlings! This is part of the exercise that you're
   supposed to solve, so open the exercise file in an editor and start your
   detective work!
3. If you're stuck on an exercise, there is a helpful hint you can view by typing
   'hint' (in watch mode), or running `rustlings hint exercise_name`.
4. If an exercise doesn't make sense to you, feel free to open an issue on GitHub!
   (https://github.com/rust-lang/rustlings/issues/new). We look at every issue,
   and sometimes, other learners do too so you can help each other out!
5. If you want to use `rust-analyzer` with exercises, which provides features like
   autocompletion, run the command `rustlings lsp`.

Got all that? Great! To get started, run `rustlings watch` in order to get the first
exercise. Make sure to have your editor open!";

const FENISH_LINE: &str = "+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                           \\/\x1b[31m
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒\x1b[0m

We hope you enjoyed learning about the various aspects of Rust!
If you noticed any issues, please don't hesitate to report them to our repo.
You can also contribute your own exercises to help the greater community!

Before reporting an issue or contributing, please read our guidelines:
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md";

const WELCOME: &str = r"       welcome to...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/";

const WATCH_MODE_HELP_MESSAGE: &str = "Commands available to you in watch mode:
  hint   - prints the current exercise's hint
  clear  - clears the screen
  quit   - quits watch mode
  !<cmd> - executes a command, like `!rustc --explain E0381`
  help   - displays this help message

Watch mode automatically re-evaluates the current exercise
when you edit a file's contents.";
