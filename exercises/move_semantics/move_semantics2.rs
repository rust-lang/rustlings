// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// done may 25

fn main() {
	// make empty Vec
    let vec0 = Vec::new();

	// make vec1 as clone of vec0 and run the function fill_vec
    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // before push of last element
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

	// after push of last element
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
