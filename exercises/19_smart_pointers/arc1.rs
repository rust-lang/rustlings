// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    
    let shared_numbers = Arc::new(numbers); // TODO
    let mut joinhandles = Vec::new();

    //The difference between Box and Arc is that Box is for single-threaded and Arc is for multi-threaded
  // what Arc new does is it takes the ownership of the data and then it creates a reference to the data and then it gives the ownership of the reference to the other threads.
  // what Arc::clone does is it increases the reference count of the data and then it gives the ownership of the reference to the other threads.

  // what happens when i use Rc instead of Arc in below code , explain with examples
    //Rc is not thread safe, so if we use Rc instead of Arc, the code will not compile.
    //Rc is used for single-threaded reference counting, while Arc is used for multi-threaded reference counting.
// can you explain the working of below code with Rc and Arc and compare?
    //The code creates a Vec of u32 called "numbers" with values ranging from 0 to 99. It then creates an Arc of the Vec, which is used to share the data across multiple threads. It then creates 8 threads, each of which sums every eighth value with an offset. The threads use the Arc to access the data, and then print the sum of the values for each offset.
    //If we use Rc instead of Arc, the code will not compile, because Rc is not thread safe. Rc is used for single-threaded reference counting, while Arc is used for multi-threaded reference counting.

    // is Arc provides mutable reference or immutable reference to threads?
    //Arc provides immutable reference to threads. It allows multiple threads to have read-only access to the data, but it does not allow any thread to have write access to the data. if we want to have mutable access to the data, we can use Mutex or RwLock in combination with Arc.
    // can we have mutable referrence usign Rc?
    //No, Rc only provides immutable reference to the data. If we want to have mutable access to the data, we can use RefCell in combination with Rc.
    //How use RefCell with Rc?
    //RefCell is a type that provides interior mutability, which means that it allows us to mutate the data even when it is behind an immutable reference. We can use RefCell in combination with Rc by wrapping the data in a RefCell, and then using Rc to share the data across multiple threads.
    // can you explain how to use RefCell with Rc and then using Rc to share the data across multiple threads?
    //To use RefCell with Rc, we first wrap the data in a RefCell, and then use Rc to share the data across multiple threads. We can then use the borrow and borrow_mut methods of RefCell to get immutable and mutable references to the data, respectively. This allows us to mutate the data even when it is behind an immutable reference, which is useful when sharing the data across multiple threads.
    // can you explain how to use Mutex with Arc and then using Arc to share the data across multiple threads with code example?
    //To use Mutex with Arc, we first wrap the data in a Mutex, and then use Arc to share the data across multiple threads. We can then use the lock method of Mutex to get a mutable reference to the data, which allows us to mutate the data safely across multiple threads. This is useful when we need to have mutable access to the data, but want to ensure that the mutations are synchronized and safe across multiple threads.
    // can you explain how to use RwLock with Arc and then using Arc to share the data across multiple threads with code example?
    //To use RwLock with Arc, we first wrap the data in a RwLock, and then use Arc to share the data across multiple threads. We can then use the read and write methods of RwLock to get immutable and mutable references to the data, respectively. This allows us to have multiple threads with read-only access to the data, or one thread with write access to the data, while ensuring that the access is synchronized and safe across multiple threads.
    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
         //output of above code:
         
// Output:
// ====================
// Sum of offset 0 is 624
// Sum of offset 4 is 576
// Sum of offset 5 is 588
// Sum of offset 6 is 600
// Sum of offset 2 is 650
// Sum of offset 1 is 637
// Sum of offset 3 is 663
// Sum of offset 7 is 612

// ====================

        //  find the difference between the above and below code's output
        //  for offset in 0..8 {
        //     let child_numbers = Arc::clone(&shared_numbers); // TODO
        //     joinhandles.push(thread::spawn(move || {
        //         let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
        //         println!("Sum of offset {} is {}", offset, sum);
        //     }).join());
               //waiting for the threads to finish after they are spawned, then only going to create another thread. 
        // }
        // for handle in joinhandles.into_iter() {
        //     handle.unwrap();
        // }
   
     // output of commented code: 

    //  Output:
    //  ====================
    //  Sum of offset 0 is 624
    //  Sum of offset 1 is 637
    //  Sum of offset 2 is 650
    //  Sum of offset 3 is 663
    //  Sum of offset 4 is 576
    //  Sum of offset 5 is 588
    //  Sum of offset 6 is 600
    //  Sum of offset 7 is 612
     
    //  ====================
     
     
}
