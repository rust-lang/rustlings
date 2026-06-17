// This program runs multiple async operations. The `Future` trait represents an
// asynchronous computation. Multiple futures can be polled concurrently with
// macros like `trpl::join!`.
use std::time::Duration;

async fn compute_value(id: u32) -> u32 {
    trpl::sleep(Duration::from_millis(10)).await;
    id * 10
}

async fn compute_all() -> Vec<u32> {
    // `trpl::join!` polls all futures concurrently and returns a tuple of results.
    let (a, b, c) = trpl::join!(compute_value(1), compute_value(2), compute_value(3));
    vec![a, b, c]
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
