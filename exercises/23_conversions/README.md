# 類型轉換

Rust 提供了多種將某一類型的值轉換為另一類型的方法。

最簡單的類型轉換形式是類型轉換表達式。它由二元運算符 `as` 表示。例如，`println!("{}", 1 + 1.0);` 這段程式碼不會編譯，因為 `1` 是整數，而 `1.0` 是浮點數。但是，`println!("{}", 1 as f32 + 1.0)` 則可以編譯。練習 [`using_as`](using_as.rs) 涵蓋了這部分內容。

Rust 還提供了實現後可用於類型轉換的特徵。這些特徵位於 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模組下。
這些特徵包括：

- `From` 和 `Into` 涵蓋在 [`from_into`](from_into.rs)
- `TryFrom` 和 `TryInto` 涵蓋在 [`try_from_into`](try_from_into.rs)
- `AsRef` 和 `AsMut` 涵蓋在 [`as_ref_mut`](as_ref_mut.rs)

此外，`std::str` 模塊提供了一個名為 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的特徵，這個特徵通過字串上的 `parse` 方法幫助將字串轉換為目標類型。如果對於給定類型 `Person` 正確實現了此特徵，那麼 `let p: Person = "Mark,20".parse().unwrap()` 不僅應該編譯，還應該在運行時不會恐慌。

這些應該是標準庫中將數據轉換為所需類型的主要方法。

## 進一步了解

這些內容沒有在書中直接涵蓋，但標準庫有詳細的文件說明。

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
