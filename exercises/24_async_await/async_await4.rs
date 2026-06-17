// This exercise demonstrates async message-passing concurrency using a channel.
use std::time::Duration;

struct Queue {
    first_half: Vec<String>,
    second_half: Vec<String>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![
                String::from("winter"),
                String::from("is"),
                String::from("really"),
                String::from("coming"),
            ],
            second_half: vec![
                String::from("we"),
                String::from("do"),
                String::from("not"),
                String::from("sow"),
            ],
        }
    }
}

async fn transmit(q: Queue, tx: trpl::Sender<String>) {
    // TODO: We want to send `tx` to both tasks. But currently, it is moved
    // into the first task (future). What change do we need to make in order to
    // solve for this issue?

    let tx_fut1 = async move {
        for val in q.first_half {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(250)).await;
        }
    };

    let tx_fut2 = async move {
        for val in q.second_half {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(tx_fut1, tx_fut2).await;
}

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let queue = Queue::new();

        let tx_fut = transmit(queue, tx);

        // TODO: Define an `rx_fut` async block that loops through all received values from `rx`
        // and processes each one.

        // TODO: Wait on the two futures (`tx_fut` and `rx_fut`) to complete.
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_messages_are_received() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();
            let queue = Queue::new();

            transmit(queue, tx).await;

            let mut received = Vec::new();
            while let Some(val) = rx.recv().await {
                received.push(val);
            }

            received.sort();

            let expected: Vec<String> =
                vec!["coming", "do", "is", "not", "really", "sow", "we", "winter"]
                    .into_iter()
                    .map(String::from)
                    .collect();

            assert_eq!(received, expected);
        });
    }
}
