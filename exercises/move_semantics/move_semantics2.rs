// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
//    let vec0 = Vec::new();
//    let mut vec1 = fill_vec(vec0.clone());

//  let  vec0 = Vec::new();
    let mut vec0 = Vec::new();
    vec0.push(99);
    //let mut vec1 = fill_vec( vec0.clone() );
//    let mut vec1 = fill_vec(&vec0);
    let mut vec1 = fill_vec(&mut vec0);
// Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
//    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//        let mut vec = vec;
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.to_vec();
fn fill_vec(vec: &mut Vec<i32>)  {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
