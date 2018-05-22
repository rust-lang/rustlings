use ansi_term::Color::{Green, Red, Yellow};
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
