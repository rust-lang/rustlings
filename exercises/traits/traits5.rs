// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than 27.

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

struct SomeStruct {
    name: String,
}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

fn some_func(item: ??) -> bool {
    item.some_function() && item.other_function()
}

fn main() {}
