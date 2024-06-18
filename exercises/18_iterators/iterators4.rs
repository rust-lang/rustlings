// iterators4.rs
//
// 執行 `rustlings hint iterators4` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // 完成此函數以回傳 num 的階乘
    // 不要使用：
    // - 提前返回（顯式使用 `return` 關鍵字）
    // 嘗試不要使用：
    // - 命令式風格的循環（for、while）
    // - 其他變量
    // 額外挑戰，不要使用：
    // - 遞迴
    // 執行 `rustlings hint iterators4` 獲取提示。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
