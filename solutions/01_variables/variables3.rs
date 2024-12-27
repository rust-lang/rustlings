#![allow(clippy::needless_late_init)]

fn main() {
    // Reading uninitialized variables isn't allowed in Rust!
    // Therefore, we need to assign a value first.
    let x: i32 = 42;

    println!("Number {x}");

    // It is possible to declare a variable and initialize it later.
    // But it can't be used before initialization.
    let y: i32;
    y = 42;
    println!("Number {y}");
}
