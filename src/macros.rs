#[macro_export]
macro_rules! title {
    ($str:expr) => {
        println!("{} {}", ansi_term::Color::Yellow.bold().paint("RUN"), $str);
    }
}

#[macro_export]
macro_rules! verify {
    ($left:expr, $right:expr) => {
        use ansi_term::Color::{Green, Red};

        if $left == $right {
            println!("{} {} == {}", Green.bold().paint("PASS"), $left, $right);
        } else {
            print!("{}", Red.bold().paint("FAIL"));
            println!(" You submitted {}, but that's not correct!", $left);
            println!("     Please correct your code to make this test pass!");
        }
    };
}

#[macro_export]
macro_rules! verify_easy {
    ($str:expr, $left:expr, $right:expr) => {
        use ansi_term::Color::{Green, Red};

        if $left == $right {
            println!("{} {}", Green.bold().paint("PASS"), $str);
        } else {
            println!("{} {}", Red.bold().paint("FAIL"), $str);
            println!("\tExpected: {}", $right);
            println!("\tGot: {}", $left);
            println!("\tPlease correct your code to make this test pass!");
        }
    };
}
