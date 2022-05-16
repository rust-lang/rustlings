// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

// // original code
// fn main() {
//     // original code
//     // needed for solution0/solution1
//     // comment out for solution2
//     let vec0 = Vec::new();

//     // solution3
//     // comment out for orignal/solution0/solution1/solution2
//     // create copy of vec0
//     let vec0_copy = vec0.clone();

//     // solution2 (part0)
//     // comment out for orignal/solution0/solution1
//     // let mut vec0 = &mut Vec::new();

//     // original code
//     // comment out for solution0/solution1/solution2/solution3
//     // let mut vec1 = fill_vec(vec0);

//     // solution0 (part0)
//     // comment out for orignal/solution1/solution2
//     // let mut vec0_ref = &vec0;

//     // solution0 (part1)
//     // comment out for orignal/solution1/solution2
//     // let mut vec1 = fill_vec(vec0_ref.to_vec());

//     // solution1 (part0)
//     // comment out for orignal/solution0/solution2
//     // let mut vec1 = fill_vec(&vec0);

//     // solution2 (part1)
//     // comment out for original/solution0/solution1
//     // fill_vec(vec0);

//     // solution3 (part1)
//     // pass vec0_copy to fill_vec
//     let mut vec1 = fill_vec(vec0_copy);

//     // original code
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     // solution2 (part2)
//     // comment out for original/solution0/solution1
//     // let mut vec1 = &mut Vec::new();
//     // fill_vec(vec1);

//     // original code
//     // needed for original and all solutions
//     vec1.push(88);

//     // original code
//     // needed for original and all solutions
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // original code
// // needed for original and solution0
// // comment out for all solution1/solution2
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// solution1 (part1)
// comment out for original/solution0/solution2
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.to_vec();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// solution2 (part0)
// comment out for original/solution0/solution1
// fn fill_vec(vec: &mut Vec<i32>) {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// }

// // original code
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // original code (cont'd)
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
// // end of original code

// solution0
// fn main() {
//     let vec0 = Vec::new();

//     // create copy of vec0
//     let vec0_copy = vec0.clone();

//     // pass vec0_copy to fill_vec
//     let mut vec1 = fill_vec(vec0_copy);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // solution0 (cont'd)
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
// // end of solution0

// // solution1
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(&vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// // solution1 (cont'd)
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     // convert borrowed vec to mutable copy
//     let mut vec = vec.to_vec().clone();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }
// // end of solution1

// solution2
fn main() {
    // create mutable ref to vec
    let mut vec0 = &mut Vec::new();

    fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

// solution2 (cont'd)
// accept mutable ref to vec as argument and act on it directly
fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
// end of solution2
