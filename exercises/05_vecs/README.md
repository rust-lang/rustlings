# 向量

向量是 Rust 中使用最廣泛的資料結構之一。在其他程式語言中，它們可能被簡單地稱為數組，但由於 Rust 操作的是較低層次的數據，數組在 Rust 中是存儲在棧上的（stack，意味著它不能增長或縮小，大小需要在編譯時知道），而向量是存儲在堆上的（heap，這些限制不適用）。

向量在書中屬於稍後的章節，但我們認為它們足夠有用，值得提前介紹。我們將在後面討論另一個有用的資料結構：雜湊表。

## 更多資訊

- [使用向量存儲值列表](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
