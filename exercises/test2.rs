// test2.rs
// This is a test for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `Strings`, some are `&strs`. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    ("blue");
    ("red".to_string());
    (String::from("hi"));
    ("rust is fun!".to_owned());
    ("nice weather".into());
    (format!("Interpolation {}", "Station"));
    (&String::from("abc")[0..1]);
    ("  hello there ".trim());
    ("Happy Monday!".to_string().replace("Mon", "Tues"));
    ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
