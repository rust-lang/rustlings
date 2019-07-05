// strings2.rs
// Make me compile without changing the function signature! Scroll down for hints :)

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}


























// Yes, it would be really easy to fix this by just changing the value bound to `word` to be a
// string slice instead of a `String`, wouldn't it?? There is a way to add one character to line
// 6, though, that will coerce the `String` into a string slice.
