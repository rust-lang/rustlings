// Let's define a simple model to track Rustlings' exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try to not use imperative loops (for/while).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` is a hash map with `String` keys and `Progress` values.
    // map = { "variables1": Complete, "from_str": None, … }
    map.values().filter(|val| **val == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        count += count_for(map, value);
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
    collection
        .iter()
        .map(|map| count_iterator(map, value))
        .sum()
}

// Equivalent to `count_collection_iterator` and `count_iterator`, iterating as
// if the collection was a single container instead of a container of containers
// (and more accurately, a single iterator instead of an iterator of iterators).
fn count_collection_iterator_flat(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    // `collection` is a slice of hash maps.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
    collection
        .iter()
        .flat_map(HashMap::values) // or just `.flatten()` when wanting the default iterator (`HashMap::iter`)
        .filter(|val| **val == value)
        .count()
}

// Equivalent to `count_collection_iterator`, `count_collection_iterator_flat` and `count_iterator`
// iterating over the collection with an accumulator in order to avoid the sum step after.
fn count_collection_iterator_fold(
    collection: &[HashMap<String, Progress>],
    value: Progress,
) -> usize {
    collection
        .iter()
        .fold(0, |acc, map| acc + count_iterator(map, value))
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use Progress::*;

    fn get_map() -> HashMap<String, Progress> {
        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Complete, Some, None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Complete), 6);
        assert_eq!(count_collection_iterator_flat(&collection, Complete), 6);
        assert_eq!(count_collection_iterator_fold(&collection, Complete), 6);
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Some), 1);
        assert_eq!(count_collection_iterator_flat(&collection, Some), 1);
        assert_eq!(count_collection_iterator_fold(&collection, Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, None), 4);
        assert_eq!(count_collection_iterator_flat(&collection, None), 4);
        assert_eq!(count_collection_iterator_fold(&collection, None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Complete, Some, None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator_flat(&collection, progress_state),
            );
        }
    }
}
