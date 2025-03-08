use tokio::time::{sleep, Duration};

// Change the function signature to be async and return a Future
async fn delayed_hello() -> String {
    // Wait for 1 second
    sleep(Duration::from_secs(1)).await;

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