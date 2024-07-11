// Importing necessary modules from async-std to handle asynchronous tasks.
use async_std::task;
use std::time::{Duration, Instant};

async fn run_async_task(id: usize) -> u128 {
    let start = Instant::now();
    // Simulate work by sleeping, but now this is asynchronous sleep.
    task::sleep(Duration::from_millis(250)).await;
    println!("Task {id} done");
    start.elapsed().as_millis()
}

async fn main() {
    let mut tasks = Vec::new();

    // Create asynchronous tasks
    for i in 0..10 {
        tasks.push(run_async_task(i));
    }

    // Wait for all tasks to complete and collect their results.
    let results = futures::future::join_all(tasks).await;

    if results.len() != 10 {
        panic!("Oh no! Some task isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Task {i} took {result}ms");
    }
}

// Main function now starts the async main function using `task::block_on`.
fn main() {
    task::block_on(main());
}
