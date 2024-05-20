// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Initialize the array to zeroes of size 100
    // Copy trait needs to be implemented by the type. In this case its primitive type.
    let a = [0; 100];
    // Explicitly initialize each element
    // let a = [0, 0, 0];
    // let a: [Option<String>; 3] = [None; 3]; alternatively let a: [Option<String>; 3] = Default::default(); if string doesn't support copy trait.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
