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
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;
    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(2));
        assert!(!is_even(3));
    }   
}
