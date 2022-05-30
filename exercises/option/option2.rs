// option2.rs
// You're gonna add if let and while let to the function below
// where the todo: are
// what's the difference between 'word' and 'optional_word'
// and how do you get one from the other?
// Make me compile! Execute `rustlings hint option2` for hints

// I AM NOT DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    word = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    integer = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}
