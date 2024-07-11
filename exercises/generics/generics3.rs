// generics3.rs
// Use your knowledge of generics to fix the `last_on_list` function signature.

// Execute `rustlings hint generics3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn last_on_list(list: &[&str]) -> &str {
    list.last().unwrap()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_str_on_list() {
        let names_list = vec!["maria", "jacob", "kacper"];
        let last = last_on_list(&names_list);
    }

    #[test]
    fn store_numbers_on_list() {
        let numbers_list = vec![1, 2, 3];
        let last = last_on_list(&names_list);
    }
}