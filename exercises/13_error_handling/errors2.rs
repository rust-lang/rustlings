// errors2.rs
//
// 假設我們在編寫一個可以用代幣購買物品的遊戲。所有物品的價格都是 5 個代幣，並且每次購買物品時都有 1 個代幣的處理費用。
// 遊戲玩家會輸入他們想購買的物品數量，而 `total_cost` 函數將計算物品的總成本。
// 由於玩家輸入的是數量，所以我們得到的是一個字串 —— 他們可能輸入了任何東西，不僅僅是數字！
//
// 現在，這個函數根本沒有處理錯誤情況（並且也沒有正確處理成功情況）。
// 我們想要的是：如果我們在一個不是數字的字符串上調用 `total_cost` 函數，那麼該函數將返回一個 `ParseIntError`，
// 在這種情況下，我們希望立即從我們的函數返回該錯誤，而不是嘗試進行乘法和加法。
//
// 至少有兩種實現方法都是正確的 —— 但其中一種要短得多！
//
// 執行 `rustlings hint errors2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
