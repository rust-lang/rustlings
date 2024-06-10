// functions4.rs
//
// 這家商店正在舉行促銷活動，如果價格是偶數，您可以獲得 10 元折扣，
// 但如果是奇數，則只折抵 3 元。（不用擔心函數本身的內容，
// 我們目前只關心函數的簽名。如果有需要，這是一個提前了解未來練習的好方法！）
//
// 執行 `rustlings hint functions4` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

fn main() {
    let original_price = 51;
    println!("您的售價是 {}", sale_price(original_price));
}

fn sale_price(price: i32) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
