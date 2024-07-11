// This program demonstrates the transformation of a synchronous function into an asynchronous one.
// The original function performs a time-consuming operation, which we'll simulate with a sleep.
// You need to convert this function to use async/await syntax and understand how it improves the responsiveness of the application.

use std::time::Duration;
use futures::executor::block_on; // This is for running the async main function in this educational context.
use tokio; // We use tokio for the async sleep functionality.

// Synchronous version of a function that simulates a long-running operation
fn calculate_value_synchronously() -> i32 {
    println!("Starting synchronous calculation...");
    std::thread::sleep(Duration::from_secs(2)); // Simulating a long-running task
    println!("Synchronous calculation done.");
    42 // returns a computed value
}

// Converted to async version
async fn calculate_value_asynchronously() -> i32 {
    println!("Starting asynchronous calculation...");
    tokio::time::sleep(Duration::from_secs(2)).await; // Using Tokio's async sleep
    println!("Asynchronous calculation done.");
    42 // returns a computed value
}

async fn main_async() {
    println!("Calling synchronous function:");
    let sync_result = calculate_value_synchronously();
    println!("Result from synchronous function: {}", sync_result);

    println!("Calling asynchronous function:");
    let async_result = calculate_value_asynchronously().await;
    println!("Result from asynchronous function: {}", async_result);
}

fn main() {
    block_on(main_async());
}
