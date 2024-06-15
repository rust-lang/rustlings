// errors5.rs
//
// 此程式使用了一個從 errors4 中修改的代碼。
//
// 本練習使用了一些我們要到課程後面才會涉及的概念，比如 `Box` 和 `From` 特徵。
// 現在並不需要詳細了解它們，但如果您願意，可以提前閱讀。現在，將 `Box<dyn ???>` 類型看作是一種“我想要實現 ??? 的任何類型”的類型，
// 考慮到 Rust 的常規運行時安全標準，這應該讓您感到有些寬鬆！
//
// 簡而言之，這個 Box 的特定用例是當您想要擁有一個值並且只關心它是一個實現特定特徵的類型時。為此，Box 被聲明為 Box<dyn Trait> 類型，
// 其中 Trait 是編譯器在該上下文中使用的任何值上查找的特徵。對於本練習，該上下文是可能在 Result 中返回的錯誤。
//
// 我們可以用什麼來描述這兩個錯誤？換句話說，有沒有一個兩個錯誤都實現的特徵？
//
// 執行 `rustlings hint errors5` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: 更新 `main()` 的返回類型以使其編譯。
fn main() -> Result<(), Box<dyn ???>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// 不要更改此行以下的任何內容。

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// 這是 `CreationError` 實現 `error::Error` 所需要的。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
