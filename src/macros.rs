#[macro_export]
macro_rules! verify {
    ($left:expr, $right:expr, $str:expr) => {
        use ansi_term::Color::{Green, Red};

        if $left == $right {
            println!("{} {}", Green.bold().paint("PASS"), $str);
        } else {
            println!("{} {}", Red.bold().paint("FAIL"), $str);
            println!("\tYou submitted {}, but that's not correct!", $left);
            println!("\tPlease correct your code to make this test pass!");
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

