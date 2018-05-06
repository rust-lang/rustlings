#[macro_use] extern crate quicli;
extern crate ansi_term;

use quicli::prelude::*;
use ansi_term::Colour::{Red, Yellow};

#[derive(Debug, StructOpt)]
struct Cli {
    exercise: Option<String>,
}

main!(|args: Cli| {
    match args.exercise {
        Some(e) => {
            println!("selected {}", e);
        }
        None => {
            println!("Welcome to {}", Yellow.paint("Rustlings"));
        }
    }
});
