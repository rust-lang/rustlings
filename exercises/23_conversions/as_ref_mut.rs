// as_ref_mut.rs
//
// AsRef 和 AsMut 允許進行廉價的引用到引用的轉換。詳細閱讀 https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html。
//
// 執行 `rustlings hint as_ref_mut` 或使用 `hint` 子命令以獲取提示。

// I AM NOT DONE

// 獲取給定參數中的字節數（不是字符數）。
// TODO: 適當地將 AsRef trait 作為 trait bound 添加。
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// 獲取給定參數中的字符數（不是字節數）。
// TODO: 適當地將 AsRef trait 作為 trait bound 添加。
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 as_mut() 將數字平方。
// TODO: 添加適當的 trait bound。
fn num_sq<T>(arg: &mut T) {
    // TODO: 實現函數主體。
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
