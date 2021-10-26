// borrowing3.rs
//
// A core safety feature of Rust is that it may not have mutable and immutable
// borrows.  Make this compile only by changing the order of the lines.

// I AM NOT DONE

fn main() {
    let mut hello = String::from("Hello ");
    let hello_ref = &hello;
    append_world(&mut hello);

    println!("{}", hello_ref);
}

fn append_world(s: &mut String) {
    s.push_str("world!");
}
