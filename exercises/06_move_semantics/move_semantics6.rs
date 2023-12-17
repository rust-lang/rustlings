// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 修改這裡，傳遞一個參考

    string_uppercase(data); // `data` 的所有權被移動到 `string_uppercase`
}

// 現在 `get_char` 接收一個對 `String` 的不可變參考
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// `string_uppercase` 保持不變，因為它需要取得所有權
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
