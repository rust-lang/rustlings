// quiz1.rs
//
// 本測驗涵蓋以下章節:
// - 變數
// - 函數
// - If
//
// Mary 在買蘋果。蘋果的價格計算如下:
// - 一個蘋果的價格是 2 rustbucks。
// - 如果 Mary 買超過 40 個蘋果，每個蘋果的價格只需 1 rustbuck!
// 寫一個函數來計算購買蘋果的總價格，給定購買的數量。
//
// 這次沒有提示 ;)

// I AM NOT DONE

// 把你的函數寫在這裡!
// fn calculate_price_of_apples {

// 不要修改這個函數!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
