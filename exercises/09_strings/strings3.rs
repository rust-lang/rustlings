// strings3.rs
//
// 執行 `rustlings hint strings3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // TODO: 去掉字串兩端的空白！
    ???
}

fn compose_me(input: &str) -> String {
    // TODO: 添加 " world!" 到字串！有多種方法可以做到這一點！
    ???
}

fn replace_me(input: &str) -> String {
    // TODO: 將字串中的 "cars" 替換為 "balloons"！
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
