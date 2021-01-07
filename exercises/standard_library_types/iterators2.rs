// iterators2.rs
// In this module, you'll learn some of the unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => format!("{}{}", first.to_uppercase(), c.collect::<String>()),
    }
}

pub fn capitalize_first_vec(vec_str: Vec<&str>) -> Vec<String> {
    let mut iter = vec_str.iter();
    iter.map(|i| capitalize_first(i)).collect()
}

pub fn capitalize_first_vec_to_str(vec_str: Vec<&str>) -> String {
    let mut iter = vec_str.iter();
    iter.map(|i| capitalize_first(i)).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalize_first_vec(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_first_vec_to_str(words);
        assert_eq!(capitalized_words, "Hello World");
    }
}
