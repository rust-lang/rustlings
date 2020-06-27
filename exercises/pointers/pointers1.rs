// pointers1.rs
// Where you pass a variable as parameters, you are not passing that variable
// instance itself, the compiler will make a copy of that variable for the use
// on that scope. 
// For manage that you need to work with a pointer variable, because you are 
// going to know exactly where the variable was allocated.
// The variables are passing on the parameters as its respective memories 
// addresses, not the values itself. 
// Make me compile! Execute `rustlings hint pointers1` for hints :)

// I AM NOT DONE

// TODO: Something is wrong on this function body
pub fn change_vals(a: &mut i32, b: &mut i32) {
    let c: i32 = *a;

    a = *b;
    b = c;
}

fn main() {
    let mut a: i32 = 5;
    let mut b: i32 = 3;

    println!("BEFORE {} e {}", a, b);

    change_vals(&mut a, &mut b);

    println!("AFTER {} e {}", a, b);
}