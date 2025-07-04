use std::mem;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // `unwrap` of an `Option` after checking if it is `None` will panic.
    // Use `if-let` instead.
    if let Some(value) = my_option {
        println!("{value}");
    }

    // A comma was missing.
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // `resize` mutates a vector instead of returning a new one.
    // `resize(0, …)` clears a vector, so it is better to use `clear`.
    my_empty_vec.clear();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Use `mem::swap` to correctly swap two values.
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
