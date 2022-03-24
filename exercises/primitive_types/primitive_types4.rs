// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // this is how you slice arrays, much like slicing python lists, but you have to borrow the variable first, and it goes &array[i..n] and you get values from i to n-1.

    assert_eq!([2, 3, 4], nice_slice)
}
