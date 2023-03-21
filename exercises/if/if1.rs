// if1.rs
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    if a>b {a}
    else if b>a {b}
    else {panic!("numbers are the same!")}
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
