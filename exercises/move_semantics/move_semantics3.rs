// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    //不可变可以绑定给可变
    //可变也可以绑定不可变
    //不可变引用借用给mutable变量，但是值仍然不可变
    //mutable 引用可以借用给不可变,同样值不变
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
