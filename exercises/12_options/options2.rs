fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Use an if-let statement to bind the value inside
        // `optional_target` to `word`.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. The loop should keep popping
        // integers from the end of the vector until there is no integer left to
        // check.
        //
        // `Vec::pop()` removes the last element from the vector and returns it,
        // or returns `None` if the vector is empty. Since this vector stores
        // `Option<i8>` values, a successful pop returns the vector's inner
        // `Option<i8>` wrapped in another `Option`.
        // You can use nested pattern matching in if-let and while-let
        // statements.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
