// vecs2.rs
//
// 給定了一個包含偶數的 Vec。您的任務是完成循環，使 Vec 中的每個數字都乘以 2。
//
// 讓我通過測試！
//
// 執行 `rustlings hint vecs2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: 填寫這裡，使 Vec `v` 中的每個元素都乘以 2。
        ???
    }

    // 在這個點，`v` 應該等於 [4, 8, 12, 16, 20]。
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: 做與上面相同的事情 - 但不修改 Vec，您可以直接返回新的數字！
        ???
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
