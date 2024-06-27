// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // When writing unit tests, it is common to import everything from the outer
    // module (`super`) using a wildcard.
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ You can assert `false` using the negation operator `!`.
    }
}
