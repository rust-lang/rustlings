// options1.rs
//
// 執行 `rustlings hint options1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

// 此函數返回冰箱裡剩下多少冰淇淋。
// 如果是晚上10點之前，還剩下5勺冰淇淋。晚上10點時，有人把它全吃光了，
// 所以不會剩下冰淇淋了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我們這裡使用24小時制，所以晚上10點是22，凌晨12點是0。
    // Option 輸出應該優雅地處理 time_of_day > 23 的情況。
    // TODO: 完成函數主體 - 記得返回一個 Option！
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: 修復此測試。您如何獲取 Option 中包含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, 5);
    }
}
