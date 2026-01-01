// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

<<<<<<< HEAD
=======
// I AM NOT DONE

#[rustfmt::skip]
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
