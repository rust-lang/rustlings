// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)
fn main() {
    let mut vec0 = Vec::new(); 

    let mut vec1 = fill_vec(&mut vec0); // We need to set line 5 to mutable, so we can mutably borrow vec0, this allows us to use vec.push in our function below

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> { 
    //here we need &mut because our input vectors are mutable
    let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec() // we need to transform vec into an immutable (type Vec not &Vec, see function defintion if you're still confused)
}