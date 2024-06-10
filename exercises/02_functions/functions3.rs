// functions3.rs
//
// 執行 `rustlings hint functions3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn main() {
    call_me();
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
