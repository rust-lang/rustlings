use tokio::task::JoinSet;

// A MultiWorker can work with the power of 5 normal workers,
// allowing us to create 5 new numbers at once!
struct MultiWorker;

impl MultiWorker {
    async fn start_work(&self) -> JoinSet<i32> {
        let mut set = JoinSet::new();

        for i in 30..35 {
            // TODO: `set.spawn` accepts an async function that will return the number
            //       we want. Implement this function as a closure!
            set.spawn(async move { i });
        }

        set
    }
}

async fn run_multi_worker() -> Vec<i32> {
    let tasks = MultiWorker.start_work().await;

    // TODO: We have a bunch of tasks, how do we run them to completion
    //       to get at the i32s they create?
    tasks.join_all().await
}

fn main() {
    // Feel free to experiment here. You may need to make some adjustments
    // to this function, though.
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_if_it_works() {
        let mut numbers = run_multi_worker().await;
        numbers.sort(); // in case tasks run out-of-order
        assert_eq!(numbers, vec![30, 31, 32, 33, 34]);
    }
}
