// macros2.rs
//
// 執行 `rustlings hint macros2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn main() {
    my_macro!();
}

macro_rules! my_macro {
    () => {
        println!("看看我的巨集！");
    };
}
