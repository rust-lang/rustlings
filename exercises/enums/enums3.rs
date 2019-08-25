// enums3.rs
// Try making an enum to immitate these structs

// Tuple Struct
struct FruitBasketRecipient(String);

// Classic Struct
struct FruitBasketSize {
    depth: i32,
    width: i32,
}

// Unit Struct
struct FruitBasketEmpty;

// Complete this definition so the file compiles
#[derive(Debug)]
enum FruitBasket {
}

fn main() {
    // These do not need to be touched :) but the file will not compile if the
    // elements of the enum have not been defines
    let recipient = FruitBasket::Recipient(String::from("John Doe"));
    println!("The fruit basket says: {:#?}", recipient);

    let size = FruitBasket::Size{depth:3, width:4};
    println!("The manual for the fruit basket says: {:#?}", size);

    let is_empty = FruitBasket::Empty;
}




























// Rust can store many different types of values in an enum, check out the 
// 'Enum Values' Subsection of the Enum chapter for more info
