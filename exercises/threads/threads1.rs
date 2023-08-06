// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

<<<<<<< HEAD
use std::sync::{Arc, Mutex};
=======
// I AM NOT DONE

>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
use std::thread;
use std::time::{Duration, Instant};

fn main() {
<<<<<<< HEAD
    // Introduce Mutex
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            // Lock mutex and make mutable
            let mut jobStatus = status_shared.lock().unwrap();
            jobStatus.jobs_completed += 1;
        }
    });
    // Not assigning should release lock before sleep
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
=======
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
    }
}
