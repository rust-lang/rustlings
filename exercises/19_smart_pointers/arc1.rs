// arc1.rs
//
// 在此練習中，我們有一個 Vec<u32> 稱為 "numbers"，其值範圍從 0 到 99 -- [0, 1, 2, ..., 98, 99]
// 我們希望在 8 個不同的線程中同時使用這組數字。每個線程將獲得每八個值的總和，並帶有一個偏移量。
//
// 第一個線程（偏移量為 0），將總結 0, 8, 16, ...
// 第二個線程（偏移量為 1），將總結 1, 9, 17, ...
// 第三個線程（偏移量為 2），將總結 2, 10, 18, ...
// ...
// 第八個線程（偏移量為 7），將總結 7, 15, 23, ...
//
// 由於我們使用的是線程，我們的值需要是線程安全的。因此，我們使用 Arc。我們需要在兩個 TODO 處進行更改。
//
// 通過在第一個 TODO 註釋處填入一個值，使這段代碼能夠編譯，並在第二個 TODO 註釋處創建一個 `child_numbers` 的初始綁定。儘量不要創建 `numbers` Vec 的任何副本！
//
// 執行 `rustlings hint arc1` 或使用 `hint` watch 子命令以獲取提示。

// I AM NOT DONE

#![forbid(unused_imports)] // 不要更改此行或下一行。
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
