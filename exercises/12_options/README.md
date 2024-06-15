# Option

`Option` 類型表示一個可選的值：每個 `Option` 要麼是 `Some` 並包含一個值，要麼是 `None`，不包含任何值。
`Option` 類型在 Rust 代碼中非常常見，因為它們有很多用途：

- 初始值
- 函數在未定義所有輸入範圍時的返回值（部分函數）
- 用於報告簡單錯誤的返回值，其中 `None` 表示錯誤
- 可選的結構體字段
- 可以被借用或“取走”的結構體字段
- 可選的函數參數
- 可空指針
- 將複雜的事情變簡單

## 進一步了解

- [Option 枚舉格式](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模組文檔](https://doc.rust-lang.org/std/option/)
- [Option 枚舉文檔](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
