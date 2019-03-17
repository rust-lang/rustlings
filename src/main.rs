use crate::run::run;
use crate::verify::verify;
use clap::{crate_version, App, Arg, SubCommand};
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::io::BufRead;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

mod run;
mod util;
mod verify;

fn main() {
    let matches = App::new("rustlings")
        .version(crate_version!())
        .author("Olivia Hugger, Carol Nichols")
        .about("Rustlings is a collection of small exercises to get you used to writing and reading Rust code")
        .subcommand(SubCommand::with_name("verify").alias("v").about("Verifies all exercises according to the recommended order"))
        .subcommand(SubCommand::with_name("watch").alias("w").about("Reruns `verify` when files were edited"))
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .about("Runs/Tests a single exercise")
                .arg(Arg::with_name("file").required(true).index(1))
                .arg(Arg::with_name("test").short("t").long("test").help("Run the file as a test")),
        )
        .get_matches();

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    if None == matches.subcommand_name() {
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
        std::process::exit(1);
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        run(matches.clone()).unwrap();
    }

    if matches.subcommand_matches("verify").is_some() {
        match verify(None) {
            Ok(_) => {}
            Err(_) => std::process::exit(1),
        }
    }

    if matches.subcommand_matches("watch").is_some() {
        watch().unwrap();
    }

    if matches.subcommand_name().is_none() {
        let mut highlighter =
            HighlightFile::new("default_out.md", &ss, &ts.themes["base16-eighties.dark"]).unwrap();
        for maybe_line in highlighter.reader.lines() {
            let line = maybe_line.unwrap();
            let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line, &ss);
            println!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        }
    }

    println!("\x1b[0m");
}

fn watch() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch("./exercises", RecursiveMode::Recursive)?;

    let _ignored = verify(None);

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("rs")) {
                        println!("----------**********----------\n");
                        let _ignored = verify(Some(b.as_path().to_str().unwrap()));
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
