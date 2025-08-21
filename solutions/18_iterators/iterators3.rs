#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }

    if a % b != 0 {
        return Err(DivisionError::NotDivisible);
    }

    Ok(a / b)
}

fn result_with_list(numbers: Vec<i64>, divisor: i64) -> Result<Vec<i64>, DivisionError> {
    //                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let division_results = numbers.into_iter().map(|n| divide(n, divisor));
    // Collects to the expected return type. Returns the first error in the
    // division results (if one exists).
    division_results.collect()
}

fn list_of_results(numbers: Vec<i64>, divisor: i64) -> Vec<Result<i64, DivisionError>> {
    //                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    let division_results = numbers.into_iter().map(|n| divide(n, divisor));
    // Collects to the expected return type.
    division_results.collect()
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
        assert_eq!(
            result_with_list(vec![27, 297, 38502, 81], 27),
            Ok(vec![1, 11, 1426, 3])
        );
        assert_eq!(
            result_with_list(vec![27, 297, 38502, 28], 27),
            Err(DivisionError::NotDivisible)
        );
        assert_eq!(
            result_with_list(vec![27, 297, 38502, 28], 0),
            Err(DivisionError::DivideByZero)
        );
        assert_eq!(
            result_with_list(vec![27, 297, i64::MIN, 28], -1),
            Err(DivisionError::IntegerOverflow)
        );
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            list_of_results(vec![27, 297, 38502, 81], 27),
            vec![Ok(1), Ok(11), Ok(1426), Ok(3)]
        );
        assert_eq!(
            list_of_results(vec![27, 297, 38502, 28], 27),
            vec![Ok(1), Ok(11), Ok(1426), Err(DivisionError::NotDivisible)]
        );
        assert_eq!(
            list_of_results(vec![27, 297, 38502, 81], 0),
            vec![
                Err(DivisionError::DivideByZero),
                Err(DivisionError::DivideByZero),
                Err(DivisionError::DivideByZero),
                Err(DivisionError::DivideByZero)
            ]
        );
        assert_eq!(
            list_of_results(vec![27, 297, i64::MIN, 81], -1),
            vec![
                Ok(-27),
                Ok(-297),
                Err(DivisionError::IntegerOverflow),
                Ok(-81)
            ]
        );
    }
}
