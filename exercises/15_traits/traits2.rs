// traits2.rs
//
// 您的任務是為字串向量實現 `AppendBar` 特徵。要實現這個特徵，想一想將 "Bar" 附加到字串向量上意味著什麼。
//
// 這次沒有模板代碼，您可以做到的！
//
// 執行 `rustlings hint traits2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 為字串向量實現 `AppendBar` 特徵。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
