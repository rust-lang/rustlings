// iterators5.rs

// Rustling progress is modelled using a hash map. The name of the exercise is
// the key and the progress is the value. Two counting functions were created
// to count the number of exercises with a given progress. These counting
// functions use imperative style for loops. Recreate this counting
// functionality using iterators.
// Execute `rustlings hint iterators5` for hints.
//
// Make the code compile and the tests pass.

// I AM NOT DONE

use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count(map: &HashMap<String, Progress>, value: Progress) -> usize {
}

fn count_stack_for(stack: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in stack {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_stack(stack: &[HashMap<String, Progress>], value: Progress) -> usize {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count(&map, Progress::Complete)
        );
    }

    #[test]
    fn count_stack_complete() {
        let stack = get_map_stack();
        assert_eq!(6, count_stack(&stack, Progress::Complete));
    }

    #[test]
    fn count_stack_equals_for() {
        let stack = get_map_stack();
        assert_eq!(
            count_stack_for(&stack, Progress::Complete),
            count_stack(&stack, Progress::Complete)
        );
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_map_stack() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
