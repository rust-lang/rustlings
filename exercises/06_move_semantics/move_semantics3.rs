// move_semantics3.rs
//
// 使我編譯通過且不新增新行 -- 只更改現有行！（不需要多個分號的行！）
//
// 執行 `rustlings hint move_semantics3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
