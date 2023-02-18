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
17) errors2 make sure you understand error propagation but also the long-form solution (just for understanding). "The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code."
18) errors3
19) HAVE to revisit error5 - > great pre-intro to Box<dyn `Trait`>
20) error6 map_err for transforming Err type in Result into a new error value of the same type.
21) Revisit error6 => HARD
22) trait3 -> very important for understanding default implementation and overrides. Also reread this page: https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations. Answer also here: https://users.rust-lang.org/t/rustlings-traits3-rs-question/80742/3
25) trait5 -> important to understand trait bounds with generics. Also be sure to understand the short format vs. clearer format and pros/cons
26) Retake quiz3 for more traits+generics practice
27) lifetimes1
28) standard library types iterators2 -> VERY IMPORTANT to understand map, chain/chain::<Vec<String>>, join
29) iterator4 to make sure you understand fold/rfold
30) iterator5 to make sure you understand looping through hashmaps with filter, flat_map, count
31) box.rs for smart pointer to allocate memory in recursive types
32) arc.rs for atomic reference counters, moving ownershup to thread::spawn.
33) rc.rs Rc::strong_count, Rc::new and Rc::clone, and drop(). Also note "In summary, if you need shared ownership in a single-threaded context, you can use Rc. If you need shared ownership across multiple threads, you should use Arc." Rc is faster and smaller than Arc
34) Cow
35) All threads exercises :(. handles and join methods for thread completion wait in the main thread.
36) threads2 exercises important for understanding synchorization primitize Mutex
37) threads3  - again - all threads exercises.
38) all macros exercises

conversions

