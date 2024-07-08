fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name, age): (&str, f32) = cat;

    println!("{name} is {age} years old");
}
