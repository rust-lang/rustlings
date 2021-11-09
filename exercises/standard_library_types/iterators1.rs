// iterators1.rs
// 
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and 
// how to go through elements within an iterable collection.
// 
// Execute `rustlings hint iterators1` for hints :D


fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 2.1
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 3
}
