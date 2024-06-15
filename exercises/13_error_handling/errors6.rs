// errors6.rs
//
// 不建議在庫代碼中使用 `Box<dyn error::Error>` 這樣的通用錯誤類型，因為調用者可能希望根據錯誤內容做出決策，而不是印出來或進一步傳播錯誤。
// 在這裡，我們定義了一個自定義錯誤類型，使得當我們的函數回傳錯誤時，調用者可以決定接下來的操作。
//
// 執行 `rustlings hint errors6` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

use std::num::ParseIntError;

// 這是一個我們將在 `parse_pos_nonzero()` 中使用的自定義錯誤類型。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // TODO: 在這裡添加另一個錯誤轉換函數。
    // fn from_parseint...
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // TODO: 修改這裡以返回合適的錯誤，而不是在 `parse()` 返回錯誤時崩潰。
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // 我們無法構造 ParseIntError，因此我們必須進行模式匹配。
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
