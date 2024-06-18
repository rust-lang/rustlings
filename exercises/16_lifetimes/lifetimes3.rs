// lifetimes3.rs
//
// 當結構體持有引用時，也需要生命週期標註。
//
// 執行 `rustlings hint lifetimes3` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
