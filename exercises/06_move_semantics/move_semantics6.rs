// move_semantics6.rs
//
// 您只能添加或刪除引用（reference），不能更改其他任何內容。
//
// 執行 `rustlings hint move_semantics6` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data);

    string_uppercase(&data);
}

// 不應該取得所有權
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// 應該取得所有權
fn string_uppercase(mut data: &String) {
    data = &data.to_uppercase();

    println!("{}", data);
}
