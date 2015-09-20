// Make me compile without changing the function signature! Scroll down for hints :)

fn main() {
    let guess1 = "blue".to_string(); // Try not changing this line :)
    let correct = guess_favorite_color(guess1);
    if correct {
        println!("You guessed correctly!");
    } else {
        println!("Nope, that's not it.");
    }
}

fn guess_favorite_color(attempt: &str) -> bool {
    attempt == "green"
}
























// Yes, it would be really easy to fix this by just changing the value bound to `guess1` to be a
// string slice instead of a `String`, wouldn't it?? There is a way to add one character to line
// 5, though, that will coerce the `String` into a string slice.
