// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    // let /* your pattern here */ = cat;
    let (name, age) = cat;
    let (name, age): (&str, f32) = cat;

    println!("{} is {} years old.", name, age);
}
