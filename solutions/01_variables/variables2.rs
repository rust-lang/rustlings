fn main() {
    // The easiest way to fix the compiler error is to initialize the
    // variable `x`. By setting its value to an integer, Rust infers its type
    // as `i32` which is the default type for integers.
    let x = 42;

    // But we can enforce a type different from the default `i32` by adding
    // a type annotation:
    // let x: u8 = 42;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
