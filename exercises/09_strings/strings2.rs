// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word //two ways in this line , i used Deref Coercion(Compiler findout what we need if Deref possible between the 2 types) to convert &String to &str
    /*word.as_str()*/  //here i used as_str() method directly to convert  to &str
) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
