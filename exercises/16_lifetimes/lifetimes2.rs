// lifetimes2.rs
//
// 如果編譯器只是驗證傳遞給註釋參數和回傳類型的引用，那麼我們需要更改什麼？
//
// 執行 `rustlings hint lifetimes2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
