// structs3.rs
//
// 結構體包含數據，但也可以包含邏輯。在這個練習中，我們定義了 Package 結構體，並且我們希望測試與其相關的一些邏輯。
// 讓代碼編譯並通過測試！
//
// 執行 `rustlings hint structs3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Package {
        if weight_in_grams < 10 {
            // 這不是在 Rust 中處理錯誤的方式，
            // 但我們稍後會學習錯誤處理。
            panic!("不能運送重量低於 10 克的包裹。")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    fn is_international(&self) -> ??? {
        // 在這裡填寫內容...
    }

    fn get_fees(&self, cents_per_gram: u32) -> ??? {
        // 在這裡填寫內容...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("西班牙");
        let recipient_country = String::from("奧地利");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("西班牙");
        let recipient_country = String::from("俄羅斯");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("加拿大");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("西班牙");
        let recipient_country = String::from("西班牙");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
