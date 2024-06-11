// options2.rs
//
// 執行 `rustlings hint options2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 將此轉換為 if let 語句，其值為 "Some" 類型
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 將此轉換為 while let 語句 - 請記住，vector.pop 也會
        // 添加另一層 Option<T>。您可以將 `Option<T>` 堆疊到 while let 和 if let 中。
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
