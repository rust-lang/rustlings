// threads2.rs
//
// 基於上一個練習，我們希望所有執行緒完成他們的工作，但這次生成的執行緒需要負責更新一個共享的值：JobStatus.jobs_completed
//
// 執行 `rustlings hint threads2` 或使用 `hint` watch 子命令以獲取提示。

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // TODO: 如果你想要一個**可變的**共享狀態，僅僅使用 `Arc` 是不夠的
    let status = Arc::new(JobStatus { jobs_completed: 0 });

    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: 在更新共享值之前，你必須採取一個動作
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有工作完成
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 印出 `JobStatus.jobs_completed` 的值
    println!("Jobs completed: {}", ???);
}
