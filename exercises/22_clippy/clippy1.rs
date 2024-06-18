// clippy1.rs
//
// Clippy 工具是一組 lint，用來分析您的程式碼，讓您能夠捕捉常見的錯誤並改進您的 Rust 程式碼。
//
// 對於這些練習，當存在 Clippy 警告時，程式碼將無法編譯。請檢查輸出中的 Clippy 建議來解決這些練習。
//
// 執行 `rustlings hint clippy1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::f32;

fn main() {
    let pi = 3.14f32;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "半徑 {:.2} 的圓的面積是 {:.5}!",
        radius, area
    )
}
