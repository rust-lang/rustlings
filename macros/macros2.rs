// macros2.rs
// Make me compile! Scroll down for hints :)

fn main() {
    my_macro!();
}

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

















































// Macros don't quite play by the same rules as the rest of Rust, in terms of
// what's available where.








// Unlike other things in Rust, the order of "where you define a macro" versus
// "where you use it" actually matters.
