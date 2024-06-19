// from_str.rs
//
// 這與 from_into.rs 類似，但這次我們將實現 `FromStr` 並返回錯誤，而不是回退到默認值。此外，在實現 FromStr 之後，你可以在字串上使用 `parse` 方法來生成實現者類型的物件。可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 閱讀更多相關資訊。
//
// 執行 `rustlings hint from_str` 或使用 `hint` 子命令來獲取提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我們將使用此錯誤類型來實現 `FromStr`。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 輸入字串為空
    Empty,
    // 字段數量不正確
    BadLen,
    // 名字字段為空
    NoName,
    // 來自 parse::<usize>() 的錯誤
    ParseInt(ParseIntError),
}

// I AM NOT DONE

// 步驟：
// 1. 如果提供的字串長度為 0，則應返回錯誤
// 2. 將給定的字串按照其中的逗號分割
// 3. 分割後應返回 2 個元素，否則返回錯誤
// 4. 從分割操作中提取第一個元素並用作名字
// 5. 從分割操作中提取另一個元素並將其解析為 `usize` 作為年齡，如 `"4".parse::<usize>()`
// 6. 如果在提取名字和年齡時出現錯誤，應返回錯誤
// 如果一切順利，則返回一個 Person 物件的 Result

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
