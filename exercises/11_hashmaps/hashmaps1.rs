// hashmaps1.rs
//
// 需要定義一個包含水果的雜湊表。鍵代表水果的名稱，值代表籃子中那種特定水果的數量。您必須在籃子中放至少三種不同類型的水果（例如蘋果、香蕉、芒果），且所有水果的總數應至少為五個。
//
// 使我能夠編譯並通過測試！
//
// 執行 `rustlings hint hashmaps1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = // TODO: 在這裡聲明您的雜湊表。

    // 已經給了您兩根香蕉 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在這裡放更多的水果到您的籃子中。

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
