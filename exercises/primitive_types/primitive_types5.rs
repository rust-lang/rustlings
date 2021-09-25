// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let tup: (&str, f32) = cat;

    println!("{} is {} years old.", tup.0, tup.1);
}
