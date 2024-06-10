// strings4.rs
//
// 好了，這裡有一堆值-- 有些是 `String`，有些是 `&str`。您的
// 任務是根據您認為每個值的類型來對每個值調用其中一個函數。
// 也就是說，在每行括號前添加 `string_slice` 或 `string`。
// 如果您正確的話，它將會編譯！
//
// 這次沒有提示！

// I AM NOT DONE

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    ???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolation {}", "Station"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("Happy Monday!".to_string().replace("Mon", "Tues"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
