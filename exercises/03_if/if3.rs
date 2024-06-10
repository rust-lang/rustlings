// if3.rs
//
// 執行 `rustlings hint if3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "螃蟹" {
        1
    } else if animal == "地鼠" {
        2.0
    } else if animal == "蛇" {
        3
    } else {
        "未知"
    };

    // 請勿更改下面的這條語句
    let habitat = if identifier == 1 {
        "海灘"
    } else if identifier == 2 {
        "地洞"
    } else if identifier == 3 {
        "沙漠"
    } else {
        "未知"
    };

    habitat
}

// 不需要更改測試。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("地鼠"), "地洞")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("蛇"), "沙漠")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("螃蟹"), "海灘")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("恐龍"), "未知")
    }
}
