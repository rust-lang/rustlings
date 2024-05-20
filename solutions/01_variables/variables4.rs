fn main() {
    // In Rust, variables are immutable by default.
    // Adding the `mut` keyword after `let` makes the declared variable mutable.
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
