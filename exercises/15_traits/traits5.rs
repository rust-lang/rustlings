// traits5.rs
//
// 您的任務是替換 '??' 部分以使代碼通過編譯。
//
// 除了標記的行外，不要更改任何行。
//
// 執行 `rustlings hint traits5` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 您只能更改下一行
fn some_func(item: ??) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
