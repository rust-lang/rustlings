// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(n) = option {
        res += n;
    }
    println!("{}", res);
}
