// Lifetimes are also needed when structs hold references.

struct Book<'a> {
    //     ^^^^ added a lifetime annotation
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
