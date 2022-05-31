// closure6.rs
// The compiler will take care of inferring the correct types for
// closures you write:
// https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-the-environment-with-closures
// But if you want to make sure that values are inaccessible after
// the closure, they can be moved rather than borrowed using the 'move' keyword.
// Make me compile!

// Execute `rustlings hint closures6` for hints!

// I AM NOT DONE

fn move_it() {
    let who_wants_to = vec!["I want to ", "He wants to ", "They want to "];
    let do_what = String::from("move it");

    let chorus: Vec<String> = who_wants_to.iter().map( move |&who| who.to_owned() + do_what.as_str() + ", " + do_what.as_str() + ".").collect();
    println!("{:?}",chorus);

    println!("{:?}!",do_what.to_uppercase());
}

fn main() {
    move_it();
}