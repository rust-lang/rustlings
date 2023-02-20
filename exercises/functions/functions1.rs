// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {

    fn hi() -> &'static str { "hi" }

    fn call_me() {
        println!("{}", hi())
    }

    call_me();
}
