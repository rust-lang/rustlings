// iterators2.rs
//
// 在這個練習中，你將學習迭代器所能提供的一些獨特優勢。按照步驟完成這個練習。
//
// 執行 `rustlings hint iterators2` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

// 步驟 1.
// 完成 `capitalize_first` 函數。
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => ???,
    }
}

// 步驟 2.
// 將 `capitalize_first` 函數應用於字串切片的切片。
// 回傳一個字串向量。
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    vec![]
}

// 步驟 3.
// 再次將 `capitalize_first` 函數應用於字串切片的切片。
// 回傳一個單一的字串。
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    String::new()
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
