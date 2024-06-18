# 特徵 (Traits)

特徵是一組方法的集合。

資料類型可以實現特徵。為此，需要為資料類型定義構成特徵的方法。例如，`String` 資料類型實現了 `From<&str>` 特徵。這使得用戶可以編寫 `String::from("hello")`。

這樣，特徵在某種程度上類似於 Java 的接口（interfaces）和 C++ 的抽象類（abstract classes）。

一些其他常見的 Rust 特徵包括：

- `Clone`（`clone` 方法）
- `Display`（允許透過 `{}` 進行格式化顯示）
- `Debug`（允許透過 `{:?}` 進行格式化顯示）

由於特徵定義了資料類型之間的共享行為，它們在編寫泛型時非常有用。

## 進一步了解

- [特徵](https://doc.rust-lang.org/book/ch10-02-traits.html)
