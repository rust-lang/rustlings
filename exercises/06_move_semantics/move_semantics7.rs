// move_semantics6.rs
//
// Here you will practice how mutable/immutable borrowing works in the context
// of a closure.
//
// Try to fix this code to make it compile and not panic.
// You can't change anything except removing 1 line.
//
// Execute `rustlings hint move_semantics7` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
  let mut counter = 0;

  let mut increment = || {
    counter += 1;
    println!("counter: {}", counter);
  };

  increment();
  let _reborrowed_counter = &counter;
  increment();

  assert_eq!(counter, 2);
}
