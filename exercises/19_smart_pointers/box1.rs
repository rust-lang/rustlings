// box1.rs
//
// 在編譯時，Rust 需要知道一個類型佔用了多少空間。這對於遞迴類型來說會變得很麻煩，因為一個值可以包含同類型的另一個值。為了解決這個問題，我們可以使用 `Box` - 一個用來在堆上存儲數據的智能指針，它還允許我們包裝一個遞迴類型。
//
// 我們在這個練習中實現的遞迴類型是 `cons 列表` - 一種在函數式程式語言中經常出現的資料結構。cons 列表中的每個項目包含兩個元素：當前項目的值和下一個項目。最後一項是一個名為 `Nil` 的值。
//
// 第一步：在枚舉定義中使用 `Box` 來使代碼能夠編譯
// 第二步：通過替換 `todo!()` 創建空和非空的 cons 列表
//
// 注意：不應該更改測試
//
// 執行 `rustlings hint box1` 或使用 `hint` watch 子命令以獲取提示。

// I AM NOT DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    println!("這是一個空的 cons 列表: {:?}", create_empty_list());
    println!(
        "這是一個非空的 cons 列表: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    todo!()
}

pub fn create_non_empty_list() -> List {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
