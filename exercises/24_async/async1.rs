use async_std::task;
use std::time::{Duration, Instant};

async fn run_async_task(id: usize) -> u128 {
    let start = Instant::now();
    // TODO: Replace the synchronous sleep with an asynchronous sleep.
    task::sleep(Duration::from_millis(250)).await;
    println!("Task {id} done");
    start.elapsed().as_millis()
}

async fn main() {
    let mut tasks = Vec::new();

    for i in 0..10 {
        // TODO: Push new asynchronous tasks into the vector.
    }

    // TODO: Wait for all tasks to complete and collect their results.

    if results.len() != 10 {
        panic!("Oh no! Some task isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Task {i} took {result}ms");
    }
}

// Main function that starts the async main function using `task::block_on`.
fn main() {
    task::block_on(main());
}
