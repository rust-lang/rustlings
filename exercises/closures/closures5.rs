// closure5.rs
// These are some fairly common uses of closures on iterator methods
// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.map
// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold
// my original version used 'reduce()' instead of 'fold()' but why would that
// be unsuitable for a sum function in Rust (specifically)?

// Execute `rustlings hint closures5` for hints!

// I AM NOT DONE

fn count_letters(animals: &Vec<&str>) -> Vec<usize>{
    // the compiler should help you figure out what signature to use
    // after you annotate it with one that fails.
    let count_closure = ;
    animals.iter().map(count_closure).collect()
}

fn sum_letters(animals: &Vec<&str>) -> usize {
    let sum_closure =
    let animals = count_letters(animals);
    // pay close attention to the where clause in the function signature
    // https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fold
    animals.iter().fold(0, sum_closure)
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
    fn test_() {
        let animals= vec!["cat","fish","horse"];
        assert_eq!(vec![3,4,5], count_letters(&animals));
        assert_eq!(12, sum_letters(&animals));
    }
}