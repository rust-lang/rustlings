// tests4.rs
//
// 確保我們正在測試正確的條件！
//
// 執行 `rustlings hint tests4` 或使用 `hint` 子命令獲取提示。

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // 只需更改測試函數本身
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("矩形的寬度和高度不能為負數！")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // 這個測試應該檢查矩形的大小是否與我們傳遞給構造函數的大小相同
        let rect = Rectangle::new(10, 20);
        assert_eq!(???, 10); // 檢查寬度
        assert_eq!(???, 20); // 檢查高度
    }

    #[test]
    fn negative_width() {
        // 這個測試應該檢查當我們嘗試創建寬度為負數的矩形時程式是否會恐慌
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        // 這個測試應該檢查當我們嘗試創建高度為負數的矩形時程式是否會恐慌
        let _rect = Rectangle::new(10, -10);
    }
}
