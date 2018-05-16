#[macro_use]
extern crate quicli;
extern crate ansi_term;

use quicli::prelude::*;
use ansi_term::Color::Yellow;

#[macro_use] mod macros;
mod about_variables;

#[derive(Debug, StructOpt)]
struct Cli {
    exercise: Option<String>,
}

main!(|args: Cli| if let Some(e) = args.exercise {
    println!("selected {}", e);
} else {
    println!("Welcome to {}!\n", Yellow.paint("Rustlings"));
    about_variables::exec();
});
