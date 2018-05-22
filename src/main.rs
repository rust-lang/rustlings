#[macro_use]
extern crate quicli;
extern crate ansi_term;

use ansi_term::Color::{Green, Red, Yellow};
use quicli::prelude::*;
use std::fmt::Display;

pub fn verify<T: PartialEq + Display>(left: T, right: T) {
    if left == right {
        println!("{} {} == {}", Green.bold().paint("PASS"), left, right);
    } else {
        println!(
            "{} You submitted {}, but that's not correct!",
            Red.bold().paint("FAIL"),
            left
        );
        println!("     Please correct your code to make this test pass!");
    }
}

pub fn verify_easy<T: PartialEq + Display>(left: T, right: T) {
    if left == right {
        println!("{} {} == {}", Green.bold().paint("PASS"), left, right);
    } else {
        println!(
            "{} You submitted {}, but that's not correct!",
            Red.bold().paint("FAIL"),
            left
        );
        println!("     Expected: {}", right);
        println!("     Please correct your code to make this test pass!");
    }
}

pub fn title(s: &str) {
    println!("{} {}", Yellow.bold().paint("RUN"), s);
}

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
