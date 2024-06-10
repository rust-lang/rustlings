// move_semantics4.rs
//
// 重構這段代碼，使得 `vec0` 不再被傳遞給 `fill_vec` 函數，
// 而是在函數內部創建 Vector 並傳回給主函數。
//
// 執行 `rustlings hint move_semantics4` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` 不再接受 `vec: Vec<i32>` 作為參數 - 不要更改這點！
fn fill_vec() -> Vec<i32> {
    // 相反，讓我們在這裡創建並填充 Vec - 您該怎麼做呢？
    let mut vec = vec;

    vec.push(88);

    vec
}
