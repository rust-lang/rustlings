fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    for x in option {
        res += x;
    }

    println!("{res}");
}
