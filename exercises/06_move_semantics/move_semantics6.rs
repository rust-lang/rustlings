// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    // get_char is taking ownership of the data
    // a reference is like a pointer.
    // it's an address to the data that we want to access.
    // the difference though is that a reference is guaranteed to point to a valid value
    // of a particular type for the life of that
    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership

//In this case, the function is taking a reference to avoid data copying
// The original string is needed after the function call.
// get_char does not modify the data, only reads it so we can just pass the reference.


fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
    // This function is getting the last character of the string reference
    // in this test case, i would be '!'
    // println!('{}'. char); would print '!'
}

// Should take ownership
// because it is modifying the value of data
// the mut in the parameters allows it to reassign a value to the 'data'
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
