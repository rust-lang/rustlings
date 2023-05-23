// generics1.rs
// Use your knowledge of generics to fix the function signature.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn last_on_list(list: &[&str]) -> &str {
    list.last().unwrap()
}

// Do not change the main method
fn main() {
    let names_list = vec!["maria", "jacob", "kacper"];
    println!("last name on the list is: {}", last_on_list(&names_list));

    let numbers_list = vec![1, 2, 3];
    println!("last number on the list is: {}", last_on_list(&numbers_list));
}
