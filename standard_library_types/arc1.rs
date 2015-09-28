// make this code compile and don't create any copies of the "numbers" Vec.
// scroll down for hints

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..5 {
        joinhandles.push(
        thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i<child_numbers.len() {
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






















// In line 6 you must create an Arc from the numbers vector.
// You must create the child_numbers inside the loop but still in the main thread.
// child_numbers is a clone of the Arc of the numbers, not a copy of the numbers.
