// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

fn main() {
    let mut shopping_list = createvec();
    shopping_list.push(54);
}
fn createvec<T>() -> Vec<T> {
    let mut shopping_list: Vec<T> = Vec::new();
    return shopping_list;
}
