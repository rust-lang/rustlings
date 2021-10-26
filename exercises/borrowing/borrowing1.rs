// borrowing1.rs

// If you want to let a function borrow a variable but not move it,
// you can use references.  This allows you to use the variable later.
// Make this compile without cloning `hello`.

// I AM NOT DONE

fn main() {
    let hello = String::from("Hello world!");
    let length = string_size(hello);

    println!("The string `{}` has {} characters", hello, length);
}

fn string_size(s: String) -> usize {
    s.len()
}
