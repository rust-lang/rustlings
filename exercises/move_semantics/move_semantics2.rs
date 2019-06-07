// move_semantics2.rs
// Make me compile without changing line 10! Scroll down for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

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














// So `vec0` is being *moved* into the function `fill_vec` when we call it on
// line 7, which means it gets dropped at the end of `fill_vec`, which means we
// can't use `vec0` again on line 10 (or anywhere else in `main` after the
// `fill_vec` call for that matter). We could fix this in a few ways, try them
// all!
// 1. Make another, separate version of the data that's in `vec0` and pass that
// to `fill_vec` instead.
// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
// and then copy the data within the function in order to return an owned
// `Vec<i32>`
// 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
// mutable), modify it directly, then not return anything. Then you can get rid
// of `vec1` entirely -- note that this will change what gets printed by the
// first `println!`
