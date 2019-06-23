// if1.rs

pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - return
    // - another function call
    // - additional variables
    // Scroll down for hints.
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}

























// It's possible to do this in one line if you would like!
// Some similar examples from other languages:
// - In C(++) this would be: `a > b ? a : b`
// - In Python this would be:  `a if a > b else b`
// Remember in Rust that:
// - the `if` condition does not need to be surrounded by parentheses
// - `if`/`else` conditionals are expressions
// - Each condition is followed by a `{}` block.
