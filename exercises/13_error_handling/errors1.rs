// errors1.rs
//
// 如果您傳遞一個空字符串，此函數將拒絕生成要印在名牌上的文本。
// 如果它能解釋一下問題所在而不是只返回 `None` 會更好。
// 幸運的是，Rust 有一個類似於 `Option` 的結構可以用來表達錯誤情況。讓我們來使用它！
//
// 執行 `rustlings hint errors1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // 不允許空名字。
        None
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // 不要更改這一行
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
