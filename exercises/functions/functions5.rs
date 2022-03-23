// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
     num * num // This is really subtle, here we're not, stricly speaking, returning anything.
                // We're simply writing an expression (because there's no ; if there was it'd be a   
                // "statement")
}
