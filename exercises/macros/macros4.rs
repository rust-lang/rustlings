// macros4.rs
// Make me compile! Scroll down for hints :)

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

// You only need to add a single character to make this compile.

// The way macros are written, it wants to see something between each
// "macro arm", so it can separate them.
