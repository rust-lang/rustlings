// async_trait.rs
//
// 异步特征是 Rust 1.75 中稳定的新特性。它允许在特征中直接定义异步方法，
// 不再需要使用 async-trait 宏。这个特性大大简化了异步编程的代码。
//
// 在这个练习中，我们将实现一个简单的异步数据获取接口。

#![allow(dead_code)]

use std::time::Duration;

// 模拟一个数据源
struct DataSource {
    data: Vec<String>,
}

impl DataSource {
    fn new() -> Self {
        Self {
            data: vec![
                "Hello".to_string(),
                "World".to_string(),
                "Rust".to_string(),
            ],
        }
    }
}

// TODO: 实现一个异步特征 AsyncDataFetcher
// 它应该包含以下异步方法：
// - fetch_data: 获取指定索引的数据
// - fetch_all: 获取所有数据
// - count: 获取数据总数
trait AsyncDataFetcher {
    async fn fetch_data(&self, index: usize) -> Option<String>;
    async fn fetch_all(&self) -> Vec<String>;
    async fn count(&self) -> usize;
}

// TODO: 为 DataSource 实现 AsyncDataFetcher 特征
impl AsyncDataFetcher for DataSource {
    async fn fetch_data(&self, index: usize) -> Option<String> {
        todo!("模拟异步获取数据，使用 tokio::time::sleep 增加延迟")
    }

    async fn fetch_all(&self) -> Vec<String> {
        todo!("模拟异步获取所有数据")
    }

    async fn count(&self) -> usize {
        todo!("模拟异步获取数据数量")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_fetch_data() {
        let source = DataSource::new();
        
        // 测试获取有效索引的数据
        assert_eq!(source.fetch_data(0).await, Some("Hello".to_string()));
        assert_eq!(source.fetch_data(1).await, Some("World".to_string()));
        
        // 测试获取无效索引的数据
        assert_eq!(source.fetch_data(10).await, None);
    }

    #[tokio::test]
    async fn test_fetch_all() {
        let source = DataSource::new();
        let all_data = source.fetch_all().await;
        
        assert_eq!(all_data, vec![
            "Hello".to_string(),
            "World".to_string(),
            "Rust".to_string(),
        ]);
    }

    #[tokio::test]
    async fn test_count() {
        let source = DataSource::new();
        assert_eq!(source.count().await, 3);
    }

    #[tokio::test]
    async fn test_concurrent_fetch() {
        let source = DataSource::new();
        
        // 测试并发获取数据
        let (data1, data2) = tokio::join!(
            source.fetch_data(0),
            source.fetch_data(1)
        );
        
        assert_eq!(data1, Some("Hello".to_string()));
        assert_eq!(data2, Some("World".to_string()));
    }
} 