// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);
}
