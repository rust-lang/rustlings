use async_std::task;
use std::time::{Duration, Instant};
use std::fmt;

// 自定义错误类型
#[derive(Debug)]
enum TaskError {
    Timeout,
    Other(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TaskError::Timeout => write!(f, "Task timed out"),
            TaskError::Other(ref err) => write!(f, "Other error: {}", err),
        }
    }
}

async fn run_async_task(id: usize) -> Result<u128, TaskError> {
    let start = Instant::now();
    // Simulate a possible failure
    if id == 5 {
        // Simulate a timeout error
        task::sleep(Duration::from_millis(100)).await;
        return Err(TaskError::Timeout);
    } else if id == 8 {
        // Simulate another type of error
        return Err(TaskError::Other("Unexpected error".to_string()));
    }

    // Normal operation with async sleep
    task::sleep(Duration::from_millis(250)).await;
    println!("Task {id} done");
    Ok(start.elapsed().as_millis())
}

async fn main() {
    let mut tasks = Vec::new();

    for i in 0..10 {
        tasks.push(run_async_task(i));
    }

    let results: Vec<Result<u128, TaskError>> = futures::future::join_all(tasks).await;

    // Process results
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(duration) => println!("Task {i} took {duration}ms"),
            Err(e) => println!("Task {i} failed with error: {}", e),
        }
    }
}

// Main function that starts the async main function using `task::block_on`.
fn main() {
    task::block_on(main());
}
