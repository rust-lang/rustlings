// closure4.rs
// Where are you going to use closures? Where the context defines the code
// you want some other code to invoke. So, unlike functions or methods, closures
// are for when the code is not generally useful, or attached to a
// type, but is specific to the context you are defining it in. Closures can
// define parameters, just like functions, but these are defined by the context
// it is intended to run in, where the actual arguments will be supplied.
// https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_by

// Execute `rustlings hint closures4` for hints!

// I AM NOT DONE

fn alphabetize(list: &mut Vec<&str>) {
    list.sort_by();
}

fn main() {
    let mut list = vec!("Oliver","Tarquinn","Bertrude");
    println!("before {:?}",list);
    alphabetize(& mut list);
    println!("after {:?}",list);
}

#[cfg(test)]
mod tests {
    use super::*;

    # [test]
    fn test_alphabetize() {
        let mut list = vec!("Oliver","Tarquinn","Bertrude");
        assert_eq!(vec!("Bertrude","Oliver","Tarquinn"),alphabetize(&mut list));
    }
}