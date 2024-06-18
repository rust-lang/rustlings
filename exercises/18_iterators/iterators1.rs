// iterators1.rs
//
// 當對集合中的元素執行操作時，迭代器是必不可少的。此模組幫助您熟悉使用迭代器的結構以及如何遍歷可迭代集合中的元素。
//
// 填寫 `???` 使我編譯通過
//
// 執行 `rustlings hint iterators1` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = ???;   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
}
