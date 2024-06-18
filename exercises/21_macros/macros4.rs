// macros4.rs
//
// 執行 `rustlings hint macros4` 或使用 `hint` watch 子指令來獲取提示。

// I AM NOT DONE

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("看看我的巨集！");
    }
    ($val:expr) => {
        println!("看看這個其他的巨集: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
