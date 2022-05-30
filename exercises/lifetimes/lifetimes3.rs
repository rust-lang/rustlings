// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Make me compile
//
// Execute the command `rustlings hint lifetimes3` if you need
// hints.

// I AM NOT DONE

struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title= String::from("Fish Flying");
    let book = Book {author: &name, title: &title};

    println!("{} by {}", book.title, book.author);
}