// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
