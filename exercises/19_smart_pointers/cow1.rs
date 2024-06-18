// cow1.rs
//
// 這個練習探索 Cow，也就是 Clone-On-Write 類型。Cow 是一個延遲克隆的智慧指標。
// 它可以封裝並提供對借用數據的不可變訪問，並在需要變異或擁有時延遲克隆數據。
// 此類型設計為通過 Borrow 特徵來處理通用的借用數據。
//
// 這個練習旨在讓你了解將數據傳遞給 Cow 時會發生什麼。通過在 TODO 標記處檢查 Cow::Owned(_) 和 Cow::Borrowed(_) 來修復單元測試。
//
// 執行 `rustlings hint cow1` 或使用 `hint` 子命令以獲取提示。

// I AM NOT DONE

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果還未擁有，則克隆到向量中。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // 由於 `input` 需要變異，發生克隆。
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("預期為擁有值"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // 由於 `input` 不需要變異，未發生克隆。
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // 我們也可以直接傳遞 `slice` 而不使用 `&`，這樣 Cow 直接擁有它。
        // 在這種情況下不發生變異，因此也不會克隆，但結果仍然是擁有的，因為它從未被借用或變異。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // 當然，如果發生變異，也是這種情況。
        // 在這種情況下，對 abs_all() 函數中的 `to_mut()` 調用返回對與之前相同數據的引用。
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
        }
    }
}
