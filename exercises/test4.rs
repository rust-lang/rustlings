// test4.rs
// This test covers the sections:
// - Modules
// - Macros
macro_rules! my_macro {
    ($val:expr) => {
        format!("Hello {}", $val);
    };
}
// Write a macro that passes the test! No hints this time, you can do it!

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
