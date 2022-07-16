// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let name = cat.0;
    let age = cat.1;

    println!("{} is {} years old.", name, age);
}
