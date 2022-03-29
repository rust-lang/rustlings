// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&*word) { // So I needed to match types here 
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool { // The type specified here needed to be respected when we passed the variable word into the function/
    attempt == "green" || attempt == "blue" || attempt == "red"
}
