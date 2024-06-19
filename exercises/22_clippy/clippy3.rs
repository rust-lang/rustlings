// clippy3.rs
// 
// 這裡有一些簡單的 Clippy 修正，你可以看到它的實用性。
// 沒有提示。

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap();
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("我的陣列! 看這裡: {:?}", my_arr);

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("這個 Vec 是空的，看到嗎? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 讓我們交換這兩個值！
    value_a = value_b;
    value_b = value_a;
    println!("值 a: {}; 值 b: {}", value_a, value_b);
}
