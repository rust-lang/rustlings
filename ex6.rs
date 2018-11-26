// ex6.rs
// Make me compile! Scroll down for hints :)

fn main() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}































// Hint: The following two statements are equivalent:
// let x = &y;
// let ref x = y;
