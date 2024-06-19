// from_into.rs
//
// From trait 用於值到值的轉換。如果 From 為某個類型正確實現，那麼 Into trait 應該可以反向工作。可以在 https://doc.rust-lang.org/std/convert/trait.From.html 閱讀更多內容。
//
// 執行 `rustlings hint from_into` 或使用 `hint` 子命令來獲取提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 我們實現 Default trait，以便在提供的字串無法轉換為 Person 物件時用作回退。
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// I AM NOT DONE

// 你的任務是完成此實現，以便讓 `let p1 = Person::from("Mark,20")` 這行代碼可以編譯。請注意，您需要將年齡部分解析為 `usize`，可以使用 `"4".parse::<usize>()` 之類的方法。需要適當處理此操作的結果。
//
// 步驟：
// 1. 如果提供的字串長度為 0，則返回 Person 的默認值。
// 2. 在字串中找到逗號並進行分割。
// 3. 從分割操作中提取第一個元素，並將其用作名字。
// 4. 如果名字為空，則返回 Person 的默認值。
// 5. 從分割操作中提取另一個元素，並將其解析為 `usize` 作為年齡。
// 如果在解析年齡時出現錯誤，則返回 Person 的默認值。否則，返回具有結果的實例化 Person 物件。

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // TODO: Complete the function body
    }
}

fn main() {
    // 使用 `from` 函數
    let p1 = Person::from("Mark,20");
    // 由於為 Person 實現了 From，因此我們應該能夠使用 Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // 測試默認的 Person 是否是 30 歲的 John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // 測試當提供錯誤的字串時是否返回 John
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // 測試 "Mark,20" 是否工作
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // 測試 "Mark,twenty" 是否會因為解析年齡錯誤回傳默認的 Person
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
