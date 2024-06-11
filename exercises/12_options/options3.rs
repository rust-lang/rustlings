// options3.rs
//
// 執行 `rustlings hint options3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("座標是 {},{} ", p.x, p.y),
        _ => panic!("沒有匹配！"),
    }
    y; // 修復此處而不刪除此行。
}
