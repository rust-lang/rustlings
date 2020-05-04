// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// I am a rooky in metaprogramming
// strongly recommend to read and practice on https://danielkeep.github.io/tlborm/book/README.html
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
