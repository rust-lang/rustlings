// generics3.rs
// Execute `rustlings hint generics3` or use the `hint` watch subcommand for a hint.

// TODO Use your knowledge of generics to enchance the `stringify` function by only changing the signature.
fn stringify(list: &[&str]) -> String {
    let items_str = list
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{items_str}]")
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
        let stringified = stringify(&names_list);
        assert_eq!(stringified, "[maria, jacob, kacper]".to_string());
    }

    #[test]
    fn store_numbers_on_list() {
        let numbers_list = vec![1, 2, 3];
        let stringified = stringify(&numbers_list);
        assert_eq!(stringified, "[1, 2, 3]".to_string());
    }
}