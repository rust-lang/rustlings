// if1.rs
//
// 執行 `rustlings hint if1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub fn bigger(a: i32, b: i32) -> i32 {
    // 完成此函數以返回較大的數字！
    // 如果兩個數字相等，可以返回任意一個。
    // 不要使用：
    // - 其他函數調用
    // - 其他變數
}

// 暫時不用管這個 :)
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

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
