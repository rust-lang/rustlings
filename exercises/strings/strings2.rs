// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
    let cool = "cool";
    let word = String::from("cool");

}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
