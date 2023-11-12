// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // let a = [0; 100];
    let a : [i32;100] = (0..100).collect::<Vec<i32>>().try_into().unwrap_or_else(|v : Vec<i32>| panic!("Expected a vec length of {} but it was {}",100,v.len()));

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
