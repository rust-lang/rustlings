// This program runs multiple async operations. The `Future` trait represents an
// asynchronous computation. Multiple futures can be polled concurrently with
// macros like `trpl::join!`.
use std::time::Duration;

async fn compute_value(id: u32) -> u32 {
    trpl::sleep(Duration::from_millis(10)).await;
    id * 10
}

async fn compute_all() -> Vec<u32> {
    // TODO: Use `trpl::join!` to await three futures concurrently.
    // Collect the results into a vector, e.g. `[10, 20, 30]`.
    todo!()
}

fn main() {
    trpl::block_on(async {
        let results = compute_all().await;
        println!("Computed: {results:?}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_all_returns_expected_values() {
        trpl::block_on(async { assert_eq!(compute_all().await, [10, 20, 30]) });
    }
}
