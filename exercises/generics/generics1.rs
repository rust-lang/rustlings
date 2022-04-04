// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // Just needed the type to expect at compile time
    shopping_list.push("milk");
}
