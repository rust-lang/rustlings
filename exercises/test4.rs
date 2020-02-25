// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

// I AM NOT DONE

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }
}

