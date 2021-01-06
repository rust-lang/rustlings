// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_value = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("The optional value doesn't contain anything!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
