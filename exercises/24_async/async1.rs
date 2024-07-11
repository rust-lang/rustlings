// This program creates multiple asynchronous tasks that each simulate a long-running operation
// using `async` and `await`. Each task will return how much time it took to complete.
// The program should wait until all the tasks have finished and should collect their return values into a vector.

use std::time::{Duration, Instant};
use tokio::time::{sleep, Duration};

async fn perform_task(id: usize) -> u128 {
    let start = Instant::now();
    // Simulate async work using sleep
    tokio::time::sleep(Duration::from_millis(250)).await;
    println!("Task {id} done");
    start.elapsed().as_millis()
}

async fn main_async() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = perform_task(i);
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Use `.await` to collect the results of all tasks into the `results` vector.
        // Remember to handle each async task using the appropriate syntax for awaiting.
    }

    if results.len() != 10 {
        panic!("Oh no! Some task isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Task {i} took {result}ms");
    }
}

fn main() {
    block_on(main_async());
}
