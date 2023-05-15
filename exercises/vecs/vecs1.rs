// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let mut v = vec![10, 20, 30, 40]; // TODO: declare your vector here with the macro for vectors
    let user1 = User {
        active: true,
        email: String::from("someone@example.com"),
        username: "test".to_string(),
        sign_in_count: 1,
    };

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
