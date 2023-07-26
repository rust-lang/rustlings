// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (&str, f64) = cat;

    println!("{} is {} years old.", name, age);
}
