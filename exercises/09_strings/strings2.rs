// strings2.rs
//
// 使我在不改變函數簽名的情況下編譯！
//
// 執行 `rustlings hint strings2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn main() {
    let word = String::from("green"); // 嘗試不要更改這行 :)
    if is_a_color_word(word) {
        println!("那是一個我知道的顏色字！");
    } else {
        println!("那不是一個我知道的顏色字。");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
