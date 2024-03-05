// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

//learn more about macros here: https://veykril.github.io/tlborm/



// #[rustfmt::skip]
/// This is a custom macro called `my_macro`.
///
/// It can be used to print messages to the console.
///
/// # Examples
///
/// ```
/// my_macro!(); // Prints: "Check out my macro!"
/// my_macro!("Hello, world!"); // Prints: "Look at this other macro: Hello, world!"
/// ```
#[rustfmt::skip]
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
