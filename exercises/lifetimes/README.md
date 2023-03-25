# Lifetimes

Lifetimes tell the compiler how to check whether references live long
enough to be valid in any given situation. For example lifetimes say
"make sure parameter 'a' lives as long as parameter 'b' so that the return
value is valid". 

They are only necessary on borrows, i.e. references, 
since copied parameters or moves are owned in their scope and cannot
be referenced outside. Lifetimes mean that calling code of e.g. functions
can be checked to make sure their arguments are valid. Lifetimes are 
restrictive of their callers.

If you'd like to learn more about lifetime annotations, the 
[lifetimekata](https://tfpk.github.io/lifetimekata/) project 
has a similar style of exercises to Rustlings, but is all about 
learning to write lifetime annotations.

## Further information

- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
