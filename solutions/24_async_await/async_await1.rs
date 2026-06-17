// This program fetches a greeting asynchronously. Async functions return a
// `Future` that must be `.await`ed to get the result. An async runtime like
// Tokio is needed to drive futures to completion.
use std::time::Duration;

async fn fetch_greeting() -> String {
    trpl::sleep(Duration::from_millis(10)).await;
    String::from("Hello, async world!")
}

fn main() {
    // `trpl::block_on()` runs a single future to completion on the Tokio `Runtime`.
    // Every time it's called, a new instance of `tokio::runtime::Runtime` will be created.
    trpl::block_on(async {
        // `.await` suspends until the future completes and yields the `String`.
        let greeting = fetch_greeting().await;
        println!("{greeting}");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_greeting_returns_expected_message() {
        trpl::block_on(async { assert_eq!(fetch_greeting().await, "Hello, async world!") });
    }
}
