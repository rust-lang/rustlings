// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// May 25/21 DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1 .. 4];
	//println!("value is {:#?}", nice_slice);

    assert_eq!([2, 3, 4], nice_slice)
}
