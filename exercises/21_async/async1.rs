// Our loyal worker works hard to create a new number.
#[derive(Default)]
struct Worker;

struct NumberContainer {
    number: i32,
}

impl Worker {
    async fn work(&self) -> NumberContainer {
        // Pretend this takes a while...
        let new_number = 32;
        NumberContainer { number: new_number }
    }
}

impl NumberContainer {
    async fn extract_number(&self) -> i32 {
        // And this too...
        self.number
    }
}

// TODO: Fix the function signature!
fn run_worker() -> i32 {
    // TODO: Make our worker create a new number and return it.
}

fn main() {
    // Feel free to experiment here. You may need to make some adjustments
    // to this function, though.
}

mod tests {
    use super::*;

    // Don't worry about this attribute for now.
    // If you want to know what this does, read the hint!
    #[tokio::test]
    async fn test_if_it_works() {
        let number = run_worker().await;
        assert_eq!(number, 32);
    }
}
