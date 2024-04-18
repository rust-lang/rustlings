// Make me compile only by reordering the lines in the test, but without adding,
// changing or removing any of them.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics5() {
        let mut x = 100;
        let y = &mut x;
        let z = &mut x;
        *y += 100;
        *z += 1000;
        assert_eq!(x, 1200);
    }
}
