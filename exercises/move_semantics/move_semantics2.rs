// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//错误1：只传引用，绑定的新 vec为引用，导致值并不可变，所以 push 会出错
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.iter().map(|x| x * 1).collect()

    // *vec1.iter().map(|x| x * 1).collect::<Vec<i32>>().to_vec()
}
