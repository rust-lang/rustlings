// const_generics.rs
// 
// Const 泛型允许我们使用编译时常量作为泛型参数。这在处理固定大小的数组和其他
// 需要编译时常量的场景特别有用。
//
// 在这个练习中，我们将实现一个简单的固定大小数组包装器，它可以安全地访问数组元素
// 并提供一些实用的方法。

#![allow(dead_code)]

// TODO: 实现一个泛型结构体 FixedArray<T, const N: usize>
// 它应该包装一个固定大小的数组 [T; N]
struct FixedArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> FixedArray<T, N> {
    // TODO: 实现 new 方法，它接受一个数组并返回 FixedArray
    fn new(arr: [T; N]) -> Self {
        todo!("创建一个新的 FixedArray 实例")
    }

    // TODO: 实现 get 方法，它安全地返回索引处的元素引用
    fn get(&self, index: usize) -> Option<&T> {
        todo!("返回指定索引处的元素，如果索引越界则返回 None")
    }

    // TODO: 实现 len 方法，返回数组的长度
    fn len(&self) -> usize {
        todo!("返回数组的长度")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_array() {
        let arr = FixedArray::new([1, 2, 3, 4, 5]);
        
        // 测试长度
        assert_eq!(arr.len(), 5);
        
        // 测试有效索引
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(4), Some(&5));
        
        // 测试无效索引
        assert_eq!(arr.get(5), None);
    }

    #[test]
    fn test_different_types() {
        let arr = FixedArray::new(["hello", "world"]);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), Some(&"hello"));
        assert_eq!(arr.get(1), Some(&"world"));
    }

    #[test]
    fn test_empty_array() {
        let arr: FixedArray<i32, 0> = FixedArray::new([]);
        assert_eq!(arr.len(), 0);
        assert_eq!(arr.get(0), None);
    }
} 