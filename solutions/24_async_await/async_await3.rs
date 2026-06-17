// Async runtimes like Tokio can spawn independent tasks that run concurrently
// on the runtime's thread pool. Each spawned task returns a `JoinHandle` that
// can be `.await`ed to get its result — similar to `thread::spawn` and `join`.
use std::time::Duration;

async fn double(n: u32) -> u32 {
    trpl::sleep(Duration::from_millis(10)).await;
    n * 2
}

async fn double_all(values: &[u32]) -> Vec<u32> {
    let mut handles = Vec::new();
    for &value in values {
        // `trpl::spawn_task` schedules the future on the runtime's executor.
        handles.push(trpl::spawn_task(double(value)));
    }

    let mut results = Vec::new();
    for handle in handles {
        // Awaiting the `JoinHandle` waits for the spawned task to finish.
        results.push(handle.await.unwrap());
    }

    results
}

fn main() {
    trpl::block_on(async {
        let results = double_all(&[1, 2, 3, 4, 5]).await;
        println!("Results: {results:?}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_all_double_values() {
        trpl::block_on(async { assert_eq!(double_all(&[1, 2, 3, 4, 5]).await, [2, 4, 6, 8, 10]) });
    }
}
