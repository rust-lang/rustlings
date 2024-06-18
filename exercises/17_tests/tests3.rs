// tests3.rs
//
// 這個測試沒有測試我們的函數——讓它以一種通過測試的方式來測試函數。然後寫第二個測試，測試當我們調用 `is_even(5)` 時是否得到預期的結果。
//
// 執行 `rustlings hint tests3` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
