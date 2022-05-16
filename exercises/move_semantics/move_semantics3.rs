// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// // original code
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // original code (cont'd)
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
// // end of original code

// solution
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// solution (cont'd)
// change vec argument to mutable
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
// end of solution
