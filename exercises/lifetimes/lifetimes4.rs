// lifetimes4.rs
//
// So that the compiler can check lifetimes of supplied attributes
//
// Make me compile
//
// Execute the command `rustlings hint lifetimes4` if you need
// hints.

// I AM NOT DONE

#[derive(Debug)]
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let book;
    {
        let title = String::from("Fish Flying");
        book = Book { author: &name, title: &title };
    }

    println!("{:?}", book);
}