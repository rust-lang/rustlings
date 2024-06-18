// iterators3.rs
//
// 這是一個比其他大多數練習都要大的練習！你能做到的！如果你選擇接受它，這是你的任務：
// 1. 完成 `divide` 函數，使前四個測試通過。
// 2. 完成 `result_with_list` 和 `list_of_results` 函數，使剩餘的測試通過。
//
// 執行 `rustlings hint iterators3` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// 計算 `a` 除以 `b`，如果 `a` 可以被 `b` 整除。
// 否則，回傳一個合適的錯誤。
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    todo!();
}

// 完成該函數並回傳正確類型的值，使測試通過。
// 期望的輸出：Ok([1, 11, 1426, 3])
fn result_with_list() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// 完成該函數並回傳正確類型的值，使測試通過。
// 期望的輸出：[Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
