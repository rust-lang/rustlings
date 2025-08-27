// functions6.rs
//
// Here you can practice special functions called `closures`, that can capture
// variables of their parent context.
// Fix the code below to make it compile, without changing the two closure
// definitions.
//
// Execute `rustlings hint functions6` or use the `hint` watch subcommand for
// some hints.

fn main() {
  // TODO: ensure the definition of captured variable
  let closure_1 = |input_var: u32| -> u32 {input_var + outer_var};
  println!("Closure#1 returns {}", closure_1(5));

  let closure_2 = |input_var| println!("Closure#2 (input_var {})", input_var);
  closure_2(2);
  closure_2("5"); // TODO: look at the captured variable type here
}
