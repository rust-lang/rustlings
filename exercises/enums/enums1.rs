// enums1.rs
// Make me compile by adding my favorite fruit in the enum definition :)
// Hints can be found below

#[derive(Debug)]
enum Fruit {
    Banana,
    Apple,
    Pear,
    // Add another fruit here
}

fn main() {
    println!("The writer likes like {:#?}", Fruit::Mango);
}




























// Look at the syntax of how the fruit is being printed to figure out what to
// add to the enum.  (HINT: I really like Mangos)
