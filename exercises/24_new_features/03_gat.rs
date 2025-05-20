// gat.rs
//
// GAT (Generic Associated Types) 是 Rust 1.65 中引入的一个强大特性。
// 它允许在关联类型中使用泛型参数，这在创建容器类型和迭代器时特别有用。
//
// 在这个练习中，我们将实现一个简单的 Map 容器，它可以存储不同类型的值
// 并提供类型安全的访问方法。

#![allow(dead_code)]

// 定义一个特征，表示可以存储和检索值的容器
trait Container {
    // TODO: 使用 GAT 定义一个关联类型 Value，它有一个生命周期参数
    type Value<'a>: 'a
    where
        Self: 'a;

    // 获取容器中的值的引用
    fn get<'a>(&'a self) -> Self::Value<'a>;
}

// 一个简单的包装类型
struct Wrapper<T>(T);

// TODO: 为 Wrapper<T> 实现 Container 特征
// Value 类型应该是对 T 的引用
impl<T> Container for Wrapper<T> {
    type Value<'a> = todo!("定义正确的关联类型");

    fn get<'a>(&'a self) -> Self::Value<'a> {
        todo!("返回对内部值的引用")
    }
}

// 一个选项包装类型
struct OptionWrapper<T>(Option<T>);

// TODO: 为 OptionWrapper<T> 实现 Container 特征
// Value 类型应该是 Option<&T>
impl<T> Container for OptionWrapper<T> {
    type Value<'a> = todo!("定义正确的关联类型");

    fn get<'a>(&'a self) -> Self::Value<'a> {
        todo!("返回 Option<&T>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper() {
        let w = Wrapper(42);
        assert_eq!(*w.get(), 42);
    }

    #[test]
    fn test_option_wrapper_some() {
        let w = OptionWrapper(Some(42));
        assert_eq!(w.get(), Some(&42));
    }

    #[test]
    fn test_option_wrapper_none() {
        let w: OptionWrapper<i32> = OptionWrapper(None);
        assert_eq!(w.get(), None);
    }

    // 这个测试确保我们的实现可以处理不同的生命周期
    #[test]
    fn test_lifetime() {
        let w = Wrapper(String::from("hello"));
        let r: &String = w.get();
        assert_eq!(r, "hello");
    }
} 