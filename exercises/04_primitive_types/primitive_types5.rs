// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // Tuples are ways to group together values with different types
    // they have fixed lengths - cannot grow or shrink
    // considered a sinle compound and it can be destructured
    // it can be more than 2 values at a time. 
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}
