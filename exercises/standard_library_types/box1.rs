// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists of by replacing `unimplemented!()`
//
// Execute `rustlings hint box1` for hints :)

// I AM NOT DONE

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let empty_list = unimplemented!();
    println!("This is an empty cons list: {:?}", empty_list);

    let non_empty_list = unimplemented!();
    println!("This is a non-empty cons list: {:?}", non_empty_list);

    // Do not change these
    assert_eq!(List::Nil, empty_list);
    assert_ne!(empty_list, non_empty_list);
}
