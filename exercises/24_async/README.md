# Async

Asynchronous programming is a model where tasks are delegated to a runtime that executes them concurrently.
It is particularly efficient for applications where many independent IO-operations are performed, e.g. web servers.

Rust provides the necessary primitives to do asynchronous programming in the language.
However, Rust's standard library does not include a runtime.
For these exercises, we will use the popular runtime called `tokio`.

## Further information

- [Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html)
- [Tokio documentation](https://docs.rs/tokio/latest/tokio/)
