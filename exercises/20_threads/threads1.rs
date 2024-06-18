// threads1.rs
//
// 這個程式會生成多個執行緒，每個執行緒至少執行250毫秒，並回傳它們完成所需的時間。
// 程式應該等待所有生成的執行緒完成，並將它們的回傳值收集到一個向量中。
//
// 執行 `rustlings hint threads1` 或使用 `hint` watch 子命令以獲取提示。

// I AM NOT DONE

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: 有一個結構體是從 thread::spawn 回傳的，你能使用它嗎？
    }

    if results.len() != 10 {
        panic!("哦不！所有生成的執行緒都沒有完成！");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} 花費了 {} 毫秒", i, result);
    }
}
