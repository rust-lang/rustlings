fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    // You can define an array with the intitial values 10, 20, 30 and 40 like
    // this:
    let a = [10, 20, 30, 40]; // Array. Do not change!

    // There is a similar way you can define a vector with initial values:
    // let v = vec![10, 20, ???]; // Vector. Needs to be fixed

    // TODO: Adjust the vector definition above so that `a` and `v` have the
    // same contents.

    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
