// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);

    let x = ['a','e','i','o','u'];
    assert_eq!(['a'], &x[0..1]);

    assert_eq!(['o','u'], &x[3..5]);
}
