// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)


fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
