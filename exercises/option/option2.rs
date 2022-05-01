// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word { // If you ignore Some() here you'll need to unwrap word
        println!("The word is: {}", word); // we don't need to unwrap wrod becasue we're using Some()
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let`
    while let Some(integer) = optional_integers_vec.pop() {// So we're wrapping integer in Some(), and this makes the types compatible for the optional integer vector
        println!("current value: {}", integer.unwrap()); // Here we need to unwrap the optional integer before printing
    }
}
