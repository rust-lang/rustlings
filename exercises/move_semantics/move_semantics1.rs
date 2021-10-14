// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    
    //esto no lo puedo hacer pq fill_vec() vec0 se ha movido a otra var, y en consecencia no existe ya vec0
    //println!("{} has length {} content `{:?}`", "vec", vec0.len(), vec0);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
