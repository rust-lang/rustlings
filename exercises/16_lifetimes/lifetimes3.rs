// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");

    // The Book struct is returning two parameters
    // so it must specify the lifetime of both references.
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
