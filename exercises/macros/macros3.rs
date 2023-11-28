// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// I AM DONE

// #[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn my_function() {
        println!("This is my function!");
    }

}

fn main() {
    my_macro!();
    macros::my_function();
}
