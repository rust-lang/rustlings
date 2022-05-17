// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// // original code
// #[test]
// fn slice_out_of_array() {
//     let a = [1, 2, 3, 4, 5];

//     let nice_slice = ???

//     assert_eq!([2, 3, 4], nice_slice)
// }
// // end of original code

// solution
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // create slice from borrowed array `a`
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
// end of solution
