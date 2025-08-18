// TODO: Fix the compiler error by moving the whole definition of this macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}


