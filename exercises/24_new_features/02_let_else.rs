// let_else.rs
//
// let-else 语句是 Rust 1.65 中引入的新特性。它允许我们在模式匹配失败时
// 提前返回或中断执行。这个特性特别适合于处理 Option 和 Result 类型。
//
// 在这个练习中，我们将使用 let-else 语句来简化错误处理代码。

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    // TODO: 使用 let-else 语句实现这个函数
    // 如果参数无效（左上角点的坐标大于右下角点的坐标），返回 None
    fn new(top_left: Point, bottom_right: Point) -> Option<Rectangle> {
        todo!("实现 Rectangle::new，使用 let-else 语句验证参数")
    }

    // TODO: 使用 let-else 语句实现这个函数
    // 函数应该解析字符串格式的矩形定义，格式为 "x1,y1,x2,y2"
    // 其中 x1,y1 是左上角坐标，x2,y2 是右下角坐标
    fn parse(s: &str) -> Option<Rectangle> {
        todo!("实现字符串解析为 Rectangle 的功能")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rectangle() {
        let rect = Rectangle::new(
            Point { x: 0, y: 0 },
            Point { x: 10, y: 10 }
        );
        assert!(rect.is_some());
    }

    #[test]
    fn test_invalid_rectangle() {
        let rect = Rectangle::new(
            Point { x: 10, y: 10 },
            Point { x: 0, y: 0 }
        );
        assert!(rect.is_none());
    }

    #[test]
    fn test_parse_valid() {
        let rect = Rectangle::parse("0,0,10,10");
        assert_eq!(rect, Some(Rectangle {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 10, y: 10 }
        }));
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(Rectangle::parse("invalid").is_none());
        assert!(Rectangle::parse("0,0,10").is_none());
        assert!(Rectangle::parse("0,0,10,10,12").is_none());
    }

    #[test]
    fn test_parse_invalid_coordinates() {
        assert!(Rectangle::parse("10,10,0,0").is_none());
    }
} 