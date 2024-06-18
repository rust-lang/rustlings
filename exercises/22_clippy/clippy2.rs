// clippy2.rs
// 
// 執行 `rustlings hint clippy2` 或使用 `hint` 子命令來獲取提示。

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);
}
