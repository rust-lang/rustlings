#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    todo!();
}

// TODO: Add the correct return type and complete the function body.
// NOTE: collect() on Iterator<Item = Result<T, E>> returns Result<Vec<T>, E>
// - If all Results are Ok: returns Ok(Vec<T>) with unwrapped values
// - If any Result is Err: stops immediately and returns that Err (fail-fast)
// Example: [9, 12, 18, 6] returns Ok([3, 4, 6, 2])
// Example: [9, 7, 12] returns Err(NotDivisible) - stops at 7, never processes 12
fn result_with_list(numbers: &[i64]) {
    let division_results = numbers.iter().map(|&n| divide(n, 3));
}

// TODO: Add the correct return type and complete the function body.
// NOTE: collect() into Vec<Result<T, E>> processes all items regardless of errors
// Example: [9, 12, 18, 6] returns [Ok(3), Ok(4), Ok(6), Ok(2)]
// Example: [9, 7, 12] returns [Ok(3), Err(NotDivisible), Ok(4)] - all processed
fn list_of_results(numbers: &[i64]) {
    let division_results = numbers.iter().map(|&n| divide(n, 3));
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
        assert_eq!(divide(81, -1), Ok(-81));
        assert_eq!(divide(i64::MIN, i64::MIN), Ok(1));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        // All numbers divisible by 3 - should succeed
        assert_eq!(result_with_list(&[9, 12, 18, 6]).unwrap(), [3, 4, 6, 2]);
    }

    #[test]
    fn test_result_with_list_error() {
        // 7 is not divisible by 3 - should fail fast at second element
        assert_eq!(
            result_with_list(&[9, 7, 12]),
            Err(DivisionError::NotDivisible)
        );
    }

    #[test]
    fn test_list_of_results() {
        // All numbers divisible by 3 - all Ok
        assert_eq!(
            list_of_results(&[9, 12, 18, 6]),
            [Ok(3), Ok(4), Ok(6), Ok(2)]
        );
    }

    #[test]
    fn test_list_of_results_with_errors() {
        // 7 is not divisible by 3 - processes all, keeps both successes and errors
        assert_eq!(
            list_of_results(&[9, 7, 12]),
            [Ok(3), Err(DivisionError::NotDivisible), Ok(4)]
        );
    }
}
