// vecs1.rs
//
// 您的任務是創建一個 `Vec`，其中包含與陣列 `a` 完全相同的元素。
//
// 讓我編譯並通過測試！
//
// 執行 `rustlings hint vecs1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 一個普通的陣列
    let v = // TODO: 使用向量的巨集在這裡宣告您的向量

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
