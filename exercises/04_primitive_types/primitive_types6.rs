// primitive_types6.rs
//
// 使用元組索引來訪問 `numbers` 的第二個元素。您可以將表達式替換到 ??? 處，使測試通過。
//
// 執行 `rustlings hint primitive_types6` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // 使用元組索引語法替換下面的 ???。
    let second = ???;

    assert_eq!(2, second,
        "這不是元組中的第二個數字！")
}
