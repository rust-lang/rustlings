// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // This block is checking whether or not optional_target is a typen  of Some(T)
        // Since it is, it's binding the value of Some(target) to word. 

        // The result is word = Some(Some(target))
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    } 

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        // Range is inclusive of the start and exclusive of the end. 
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
       while let Some(Some(integer)) = optional_integers.pop() {
            // Removes the last character for the vector and return it
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
