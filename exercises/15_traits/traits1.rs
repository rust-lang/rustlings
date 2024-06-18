// traits1.rs
//
// 是時候實現一些特徵了！您的任務是為 `String` 類型實現 `AppendBar` 特徵。
// `AppendBar` 特徵只有一個函數，它將 "Bar" 附加到任何實現了該特徵的對象中。
//
// 執行 `rustlings hint traits1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: 為 `String` 類型實現 `AppendBar`。
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
