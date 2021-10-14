// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    let mut word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word ({}) I know!", &word);
    } else {
        println!("That is not a color word I know.");
    }

    word = String::from("red");
    if is_a_color_word(&word) {
        println!("That is a color word ({}) I know!", &word);
    } else {
        println!("That is not a color word I know.");
    }

}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
