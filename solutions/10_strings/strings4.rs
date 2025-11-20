fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // Here, both answers work.
    // `.into()` converts a type into an expected type.
    // If it is called where `String` is expected, it will convert `&str` to `String`.
    string("nice weather".into());
    // But if it is called where `&str` is expected, then `&str` is kept `&str` since no conversion is needed.
    // If you remove the `#[allow(…)]` line, then Clippy will tell you to remove `.into()` below since it is a useless conversion.
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
