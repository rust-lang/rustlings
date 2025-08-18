// generics3.rs
// Execute `rustlings hint generics3` or use the `hint` watch subcommand for a hint.

// This function should take an array of `Option` elements and returns array of not None elements
// TODO fix this function signature
fn into_dispose_nulls(list: Vec<Option<&str>>) -> Vec<&str> {
    list.into_iter().flatten().collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_str_on_list() {
        let names_list = vec![Some("maria"), Some("jacob"), None, Some("kacper"), None];
        let only_values = into_dispose_nulls(names_list);
        assert_eq!(only_values.len(), 3);
    }

    #[test]
    fn store_numbers_on_list() {
        let numbers_list = vec![Some(1), Some(2), None, Some(3)];
        let only_values = into_dispose_nulls(numbers_list);
        assert_eq!(only_values.len(), 3);
    }

    #[test]
    fn store_custom_type_on_list() {
        #[allow(dead_code)]
        struct Rectangle {
            width: i32,
            height: i32,
        }
        impl Rectangle {
            fn new(width: i32, height: i32) -> Self {
                Self { width, height }
            }
        }

        let custom_list = vec![
            Some(Rectangle::new(1, 2)),
            None,
            None,
            Some(Rectangle::new(3, 4)),
        ];
        let only_values = into_dispose_nulls(custom_list);
        assert_eq!(only_values.len(), 2);
    }
}
