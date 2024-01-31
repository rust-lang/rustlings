// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone());

    // When vec0 is passed to fill_vec, the function takes OWNERSHIP 
    // of the vector. It mutates the vector and passes ownershpip to vec1. 
    // Once a value has moved, we can no longer use it. 
    // this is why, we need to clone it so that we can use it again

    // Using clone() makes an entire copy of the vector. DEEP COPY. 
    // It is also a separate object in memory. 
    // Changes to the original vector does not affect the cloned vector and vice versa

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]); 
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
