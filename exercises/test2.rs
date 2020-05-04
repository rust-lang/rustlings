// test2.rs
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
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); //https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
    string("nice weather".into());
    string(format!("Interpolation {}", "Station")); //format Marco println String type https://doc.rust-lang.org/std/fmt/
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //change thevalue and it will create a new String https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
}
