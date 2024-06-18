// lifetimes1.rs
//
// Rust編譯器需要知道如何檢查提供的引用是否有效，以便讓程式設計師知道引用是否有在使用前超出範圍的風險。記住，引用是借用，不擁有它們自己的值。如果它們的所有者超出範圍怎麼辦？
//
// 執行 `rustlings hint lifetimes1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
