// structs1.rs
//
// 完成所有的 TODO 項以通過測試！
//
// 執行 `rustlings hint structs1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

struct ColorClassicStruct {
    // TODO: 在此處添加內容
}

struct ColorTupleStruct(/* TODO: 在此處添加內容 */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: 實例化一個經典 C 結構體！
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 實例化一個元組結構體！
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 實例化一個單元結構體！
        // let unit_like_struct =
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
