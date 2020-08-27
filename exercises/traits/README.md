### Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

+ `Clone` (the `clone` method),
+ `Display` (which allows formatted display via `{}`), and
+ `Debug` (which allows formatted display via `{:?}`).

Because traits indicate shared behavior between data types, they are useful when writing generics.


#### Book Sections

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
