// strings3.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // String slice references part of another data, and does not own the data
    // Starts as reference to static string in code
    string_slice("blue");
    // Create String object from a static string
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    // Interpolation used to combine multiple strings
    string(format!("Interpolation {}", "Station"));
    // Use of & or other functions to slice parts of a String
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    // Mutation of a String object / data
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
