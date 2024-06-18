// traits3.rs
//
// 您的任務是為兩個結構實現 Licensed 特徵，並讓它們回傳相同的訊息，而不需要兩次編寫相同的函數。
//
// 您考慮一下可以向 Licensed 特徵添加點什麼。
//
// 執行 `rustlings hint traits3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 不要編輯這一行
impl Licensed for OtherSoftware {} // 不要編輯這一行

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
