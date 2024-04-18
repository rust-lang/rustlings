// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterators() {
        let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

        let mut my_iterable_fav_fruits = ???;   // TODO: Step 1

        assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
        assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 2
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
        assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 3
        assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
        assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Step 4
    }
}
