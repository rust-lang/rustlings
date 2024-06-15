// errors3.rs
//
// 這是一個試圖使用前一個練習中完成的 `total_cost` 函數的程序。但它不起作用！
// 為什麼不行？我們應該怎麼做才能修復它？
//
// 執行 `rustlings hint errors3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("您買不起！");
    } else {
        tokens -= cost;
        println!("您現在有 {} 個代幣。", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
