// Modify delayed_hello to return the string "Hello, world!"
// after waiting for 1 second to pass the test and fix the
// compiler error.

use tokio::time::{sleep, Duration};

// TODO: Change the function signature to fix the compiler error
fn delayed_hello() -> String {
    // TODO: Return the string "Hello, world!" after waiting for 1 second
    // ...

    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
    // You can experiment optionally here
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_delayed_hello() {
        let start = std::time::Instant::now();
        let result = delayed_hello().await;
        let duration = start.elapsed();
        assert_eq!(result, "Hello, world!");
        assert!(duration >= Duration::from_secs(1));
        println!("Test passed!");
    }
}