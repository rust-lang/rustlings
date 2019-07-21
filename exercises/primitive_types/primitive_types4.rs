// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the `if` statement
// returns true. Scroll down for hints!!

fn main() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..3];

    if nice_slice == [2, 3, 4] {
        println!("Nice slice!");
    } else {
        println!("Not quite what I was expecting... I see: {:?}", nice_slice);
    }
}

























// Take a look at the Understanding Ownership -> Slices -> Other Slices section of the book:
// https://doc.rust-lang.org/book/ch04-03-slices.html
// and use the starting and ending indices of the items in the Array
// that you want to end up in the slice.

// If you're curious why the right hand of the `==` comparison does not
// have an ampersand for a reference since the left hand side is a
// reference, take a look at the Deref coercions section of the book:
// https://doc.rust-lang.org/book/ch15-02-deref.html
