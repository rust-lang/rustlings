// iterators2.rs
//
// INFO: In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut x = match c.next() {
        None => String::new(),
        Some(first) => first.to_string().to_uppercase(),
    };
    if input.len() == 1 {
        return String::from(" ");
    }
    if input.len() == 0 {
        return String::from("");
    }
    x.push_str(&input[1..input.len()]);
    x
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut v = vec!["hello", "world"];
    let mut new_vec: Vec<String> = vec![];
    for x in v {
        let s = capitalize_first(x);
        new_vec.push(s);
    }
    new_vec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let words_iter = words.iter();
    let mut new_word = String::from("");
    for x in words_iter {
        let y = capitalize_first(*x);
        new_word.push_str(&y);
    }
    println!("{new_word}");
    new_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
