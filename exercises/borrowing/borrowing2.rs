// borrowing2.rs
//
// Rust has strong guarantees about immutability.  In order for something
// to be mutated, it must be marked explicitly.  Make this compile only
// by changing the mutabilities of the calls.

// I AM NOT DONE

fn main() {
    let hello = String::from("Hello ");
    append_world(&hello);

    assert_eq!(&hello, "Hello world!");
}

fn append_world(s: &String) {
    s.push_str("world!");
}
