// enums4.rs
//
// Address all the TODOs to make the tests pass!

// Execute `rustlings hint enums4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE



#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Yellow
    
}
fn main() {
    let color_lk = Color::Blue;
    let Color_ll = Color :: Yellow;
    match color_lk {
        Color::Red => println!("{:?}",Color::Red),
        Color::Blue => println!("{:?}",Color::Blue),
        Color::Yellow => println!("{:?}",Color::Yellow)
        
    };
    match  Color_ll{
        Color::Red => println!("{:?}",Color::Red),
        Color::Blue => println!("{:?}",Color::Blue),
        Color::Yellow => println!("{:?}",Color::Yellow)
        
    }
}
