#![allow(clippy::ptr_arg)]

// Borrows instead of taking ownership.
// It is recommended to use `&str` instead of `&String` here. But this is
// enough for now because we didn't handle strings yet.
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Takes ownership instead of borrowing.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
