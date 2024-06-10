// if2.rs
//
// 第一步：讓我編譯通過！
// 第二步：讓 bar_for_fuzz 和 default_to_baz 測試通過！
//
// 執行 `rustlings hint if2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else {
        1
    }
}

// 不需要更改測試！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
