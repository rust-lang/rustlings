// macros3.rs
//
// 讓我能夠編譯通過，不用把巨集移出模組！
//
// 執行 `rustlings hint macros3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

mod macros {
    macro_rules! my_macro {
        () => {
            println!("看看我的巨集！");
        };
    }
}

fn main() {
    my_macro!();
}
