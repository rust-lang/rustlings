// using_as.rs
//
// 在 Rust 中，類型轉換是通過使用 `as` 運算符完成的。請注意，`as` 運算符不僅僅用於類型轉換。它還有助於重命名導入項。
//
// 目標是確保除法不會失敗並返回正確的類型。
//
// 執行 `rustlings hint using_as` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
