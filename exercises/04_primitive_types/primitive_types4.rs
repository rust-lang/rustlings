// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    // XXX Reason you don't need to add '&' infront of the slice [2,3,4] is
    // deref trait is implicit in the current context. 
    // If we define a new data type and define our own deref trait, the same behavior can be expected.
    assert_eq!([2, 3, 4], nice_slice)
}
