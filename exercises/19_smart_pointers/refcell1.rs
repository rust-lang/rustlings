// refcell1.rs
//
// Interior mutability is a design pattern in Rust that allows you to mutate 
// data even when there are immutable references to that data; 
// normally, this action is disallowed by the borrowing rules.  

// The RefCell<T> type represents single ownership over the data it holds.
// Recall the borrowing rules in Rust:
//      1. At any given time, you can have either (but not both) one mutable 
//      reference or any number of immutable references.
//      2. References must always be valid.

// With references and Box<T>, the borrowing rules’ invariants are enforced at 
// compile time. With RefCell<T>, these invariants are enforced at runtime. 
// With references, if you break these rules, you’ll get a compiler error. 
// With RefCell<T>, if you break these rules, your program will panic and exit.
// The RefCell<T> type is useful when you’re sure your code follows the 
// borrowing rules but the compiler is unable to understand and guarantee that.

// I AM NOT DONE

use std::cell::RefCell;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    name: RefCell<String>,
}

#[allow(dead_code)]
impl User {
    fn name(&self) -> String {
        self.name.borrow().to_string()
    }

    // Note: do not use &mut self!
    fn set_name(&self, name: String) {
        todo!()
    }
}

fn main() {
    let u = User {
        name: RefCell::new("Alice".to_string()),
    };
    println!("My name is {}!", *u.name.borrow());

    let new_name = "Bob".to_string();
    u.set_name(new_name.clone());

    println!("My name is {}!", *u.name.borrow());
}
