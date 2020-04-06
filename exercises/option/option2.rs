// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    let optional_value = String::from("rustlings");
    if let Some(value) = optional_value {
        println!("the value of optional value is: {}", value);
    } else {
        println!("optional value does not have a value!");
    }

    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(x);
    }
    while let Some(Some(value)) = optional_values_vec.pop() {
        println!("current value: {}", value);
    }
}
