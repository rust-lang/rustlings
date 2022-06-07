// closure5.rs
// These are some fairly common uses of closures on iterator methods
// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map
// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.reduce
// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold

// Execute `rustlings hint closures5` for hints!

// I AM NOT DONE

fn count_letters(animals: &Vec<&str>) -> Vec<usize>{
    // the compiler should help you figure out what signature to use
    // after you annotate it with one that fails.
    // TODO: fill in this line with closure that compiles and passes the test.
    let count_closure = ;
    animals.iter().map(count_closure).collect()
}

fn sum_letters(animals: &Vec<&str>) -> usize {
    let animals = count_letters(animals);
    // pay close attention to the where clause in the function signatures
    // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.reduce
    // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold
    // and the return types of the different ways of getting an iterator
    // https://doc.rust-lang.org/stable/std/iter/
    // https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter
    // TODO: change the next 2 lines to compile and pass the test.
    let sum_closure = |x: &usize, y: &usize| &(x + y);
    animals.iter().reduce(sum_closure).unwrap().to_owned()
}

fn main() {
    let animals= vec!["cat","fish","horse"];
    println!("animals: {:?}",animals);
    println!("length: {:?}", count_letters(&animals));
    println!("total: {:?}", sum_letters(&animals));
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_closures() {
        let animals= vec!["cat","fish","horse"];
        assert_eq!(vec![3,4,5], count_letters(&animals));
        assert_eq!(12, sum_letters(&animals));
    }
}