// move_semantics1.rs
//
// 執行 `rustlings hint move_semantics1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;

    vec.push(88);

    vec
}
