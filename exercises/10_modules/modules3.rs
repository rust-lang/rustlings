// modules3.rs
//
// 您可以使用 'use' 關鍵字從任何地方，特別是從 Rust 標準庫中引入模組路徑到您的作用域中。從 std::time 模組引入 SystemTime 和 UNIX_EPOCH。
// 如果您能用一行完成它，會有額外的風格分！
//
// 執行 `rustlings hint modules3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

// TODO: 完成這個 use 語句
use ???

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC 到現在已經過去 {} 秒了！", n.as_secs()),
        Err(_) => panic!("SystemTime 早於 UNIX EPOCH！"),
    }
}
