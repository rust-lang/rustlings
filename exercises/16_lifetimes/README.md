# 生命週期

生命週期用於告訴編譯器如何檢查引用是否有足夠長的存續時間，以確保它們有效。例如，生命週期可以說「確保參數 'a' 的存活時間至少與參數 'b' 一樣長，這樣回傳的值才是有效的」。

它們僅對借用（即引用）是必要的，因為被複製的參數或移動的參數在它們的範圍內是擁有的，不能在範圍外引用。生命週期意味著可以檢查函數等的調用代碼，以確保它們的參數是有效的。生命週期對其調用者是有約束力的。

如果您想了解更多關於生命週期註釋的資訊，[lifetimekata](https://tfpk.github.io/lifetimekata/) 項目提供了一種類似於 Rustlings 的練習風格，但全都是關於學習如何編寫生命週期註釋。

## 進一步了解

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命週期驗證引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
