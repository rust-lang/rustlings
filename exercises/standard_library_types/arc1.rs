// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and creating an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Scroll down for hints :)

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}




















// Make `shared_numbers` be an `Arc` from the numbers vector. Then, in order
// to avoid creating a copy of `numbers`, you'll need to create `child_numbers`
// inside the loop but still in the main thread.

// `child_numbers` should be a clone of the Arc of the numbers instead of a
// thread-local copy of the numbers.
