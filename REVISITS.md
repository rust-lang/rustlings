1) primitive_types4.rs - slices and arrays
2) Revisit macros (good explanation here: https://doc.rust-lang.org/rust-by-example/macros.html)
3) vecs2.rs -> review v.iter_mut to mutate vector elements vs. let out = v.iter().map(|n| n*2).collect() to multiple each value but
return results to new Vec<i32> vector
4) move_semantics1 why does vec0 -> v inside the function allow v to be mutable? Analyze in detail. [Actually this is called a move of ownership!]
5) move_semantics2 -> important!!!
6) structs3 -> review impl (for struct implementations it adds StructName::method() methods to it (kind of like a value receiver in Go :) . Keep in mind there are also trait (interface) impls.
7) enums2 and enums3 are VERY important to understand enum matching!!!
8) strings3 check differences between str and String libraries. Yes str is a string slice but how to know when to use String methods?
9) modules all -> important for understanding modules
10) insert method for hashmaps (see hashmaps1)
11) hashmap3 is good for understanding entry().or_insert() pattern for hashmaps
12) generics2. review https://doc.rust-lang.org/book/ch10-01-syntax.html
13) traits1 why is self accepted since it's being updated?
14) revisit all traits exercises
15) options1 (understand Some and None)
16) option2 (important to understand if let, while let statements alternative to match ). ChatGPT explanation was great.

