// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.


use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}
//what is the difference between Strong and Weak count?
//Strong count is the number of Rc pointers to the data, and Weak count is the number of Weak pointers to the data.
//Rc::strong_count(&sun) returns the number of strong (Rc) pointers to the data, and Rc::weak_count(&sun) returns the number of weak (Weak) pointers to the data.
// what is Rc pointers and Weak pointers?
//Rc pointers are reference counted pointers, which means that they keep track of the number of references to the data they point to, and automatically deallocate the data when the number of references drops to zero.
//Weak pointers are similar to Rc pointers, but they do not keep the data alive. They are used to break reference cycles, where two or more objects reference each other, and thus keep each other alive, even though they are no longer needed.
//Rc pointers are used to keep the data alive, while Weak pointers are used to break reference cycles.
// what is reference cycles?
//Reference cycles occur when two or more objects reference each other, and thus keep each other alive, even though they are no longer needed. This can lead to memory leaks, where memory is not deallocated even though it is no longer needed.\
//give example of reference cycles?
//An example of a reference cycle is a parent and a child object that reference each other. If the parent object has a reference to the child object, and the child object has a reference to the parent object, then they will keep each other alive, even though they are no longer needed.
// how to find out weak count using Rc?
//Rc::weak_count(&sun) returns the number of weak (Weak) pointers to the data.
// what is the use of Rc::clone()?
//Rc::clone() is used to create a new reference to the same data. It increases the reference count, so that the data is not deallocated until all references are dropped.

#[test]
fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    // TODO
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    // TODO
    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    // TODO
    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    // TODO
    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
