// strings3.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings3` for hints ;)

// I AM NOT DONE

fn main() {
    let hello = String::from("hello");
    let suffix = " there!";

    let result = append_str(hello, suffix);

    assert_eq!(result, "hello there!".to_string());
    println!("{}", result);

    let lo = String::from("lo");
    let result = append_char(lo, 'l');

    assert_eq!(result, "lol");
    println!("{}", result);
}

fn append_str(mut s: String, suffix: &str) -> String {
    // TODO: append the suffix to s

    s
}

fn append_char(mut s: String, suffix: char) -> String {
    // TODO append the suffix to s

    s
}
