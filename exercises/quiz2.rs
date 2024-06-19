// quiz2.rs
//
// 本測驗涵蓋以下章節:
// - 字串
// - 向量 (Vec)
// - 移動語義 (Move semantics)
// - 模組 (Modules)
// - 枚舉 (Enums)
//
// 讓我們建立一個函數形式的小機器。作為輸入，我們將提供一個字串和命令的列表。
// 這些命令決定了要對字串執行的操作。它可以是:
// - 將字串轉為大寫
// - 修剪字串
// - 將 "bar" 附加到字串指定的次數
// 具體形式如下:
// - 輸入是一個長度為2的元組向量，
//   第一個元素是字串，第二個是命令。
// - 輸出是一個字串向量。
//
// 這次沒有提示!

// I AM NOT DONE

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 完成函數簽名!
    pub fn transformer(input: ???) -> ??? {
        // TODO: 完成輸出聲明!
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: 完成函數體。你能做到的！
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: 我們需要導入什麼來讓 `transformer` 在作用域內?
    use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
