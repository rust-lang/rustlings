// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// // original code
// fn main() {
//     let data = "Rust is great!".to_string();

//     get_char(data);

//     string_uppercase(&data);
// }

// // original code (cont'd)
// // Should not take ownership
// fn get_char(data: String) -> char {
//     data.chars().last().unwrap()
// }

// // original code (cont'd)
// // Should take ownership
// fn string_uppercase(mut data: &String) {
//     data = &data.to_uppercase();

//     println!("{}", data);
// }
// // end of original code

// solution
fn main() {
    let data = "Rust is great!".to_string();

    // change passed data to ref so as to not take ownership of `data`
    get_char(&data);

    // remove ref so string_uppercase takes ownership of `data`
    string_uppercase(data);
}

// solution (cont'd)
// Should not take ownership
// add ref to argument type
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// solution (cont'd)
// Should take ownership
// remove ref from argument type
fn string_uppercase(mut data: String) {
    // remove ref from data
    data = data.to_uppercase();

    println!("{}", data);
}
// end of solution
