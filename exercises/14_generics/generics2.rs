// generics2.rs
//
// 這個強大的包裝器提供了存儲正整數值的能力。
// 使用泛型重寫它，使其支援包裝任何類型。
//
// 執行 `rustlings hint generics2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

struct Wrapper {
    value: u32,
}

impl Wrapper {
    pub fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
