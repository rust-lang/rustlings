// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, mytx: Arc<Mutex<mpsc::Sender<u32>>>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    let tx1 = Arc::clone(&mytx);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            let mytx = tx1.lock().unwrap();
            mytx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx2 = Arc::clone(&mytx);

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            let mytx = tx2.lock().unwrap();
            mytx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let newtx = Arc::new(Mutex::new(tx));
    send_tx(queue, newtx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
