#[macro_use] extern crate quicli;
extern crate ansi_term;

use quicli::prelude::*;
use ansi_term::Colour::{Red, Yellow, Green};

macro_rules! verify {
    ( $str:expr, $left:expr, $right:expr ) => {
        if ($left == $right) {
            println!("{} {}", Green.bold().paint("PASS"), $str);
        } else {
            println!("{} {}", Red.bold().paint("FAIL"), $str);
            println!("\tYou submitted {}, but that's not correct!", $left);
            println!("\tPlease correct your code to make this test pass!");
        }
    }
}

macro_rules! verify_easy {
    ( $str:expr, $left:expr, $right:expr ) => {
        if ($left == $right) {
            println!("{} {}", Green.bold().paint("PASS"), $str);
        } else {
            println!("{} {}", Red.bold().paint("FAIL"), $str);
            println!("\tExpected: {}", $right);
            println!("\tGot: {}", $left);
            println!("\tPlease correct your code to make this test pass!");
        }
    }
}

mod about_variables;

#[derive(Debug, StructOpt)]
struct Cli {
    exercise: Option<String>,
}

main!(|args: Cli| {
    if let Some(e) = args.exercise {
        println!("selected {}", e);
    } else {
        println!("Welcome to {}", Yellow.paint("rustlings"));
        verify!("One equals one", 1, 2);
    }
});
