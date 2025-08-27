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

fn main() {
  let mut counter = 0;

  let mut increment = || {
    counter += 1;
    println!("counter equals {}", counter);
  };

  increment();
  let _reborrowed_counter = &counter; // TODO: figure out where to put this borrowing instruction
  increment();

  assert_eq!(counter, 2);
}
