// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// may 25/21 DONE

fn main() {
    // create array of type i32 having 120 elements
    let a: [i32; 120] = [0; 120];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        //println!("the number of elements in array: {}", a.len());
		//make function to borrow whole array as slice
		analyze_slice(&a);
		//slice can point to a section of array
		println!("borrow a section of the array as a slice");
		analyze_slice(&a[0 .. 10]);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
// this function borrows a slice
fn analyze_slice(slice: &[i32]) {
	println!("slice first element: {}", slice[0]);
	println!("borrowed slice has {} elements", slice.len());

}