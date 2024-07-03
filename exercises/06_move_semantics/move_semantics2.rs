// move_semantics2.rs
//
// 使測試通過，找到一種方法讓兩個 Vec 保持分離！
//
// 執行 `rustlings hint move_semantics2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
