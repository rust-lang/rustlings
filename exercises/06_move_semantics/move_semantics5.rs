// move_semantics5.rs
//
// 使我編譯通過，只需重新排列 `main()` 中的行，但不添加、更改或刪除其中任何一行。
//
// 執行 `rustlings hint move_semantics5` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
