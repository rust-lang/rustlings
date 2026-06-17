# Async/Await

Rust's async model lets you write concurrent code that waits for I/O or other
work without blocking a thread. An `async fn` returns a `Future` — a value that
represents work still in progress. The `.await` keyword pauses until that work
finishes.

Futures don't run on their own: an **async runtime** (such as [Tokio](https://tokio.rs))
polls them to completion. Rustlings uses Tokio (via the [`trpl`](https://crates.io/crates/trpl) crate) for these exercises.

## Further information

- [The Rust Programming Language: Async Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Async Programming Book](https://rust-lang.github.io/async-book/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
