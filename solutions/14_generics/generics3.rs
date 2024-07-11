// generics3.rs
// Execute `rustlings hint generics3` or use the `hint` watch subcommand for a hint.

// Here we add generic in function declaration so function can work with different types
fn stringify<T>(list: &Vec<T>) -> String 
where
    T: ToString // here we also specify that T needs to implement ToString trait so we can use .to_string() on the vetor elements
{
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