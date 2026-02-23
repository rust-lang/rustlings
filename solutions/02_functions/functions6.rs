fn main() {
    let outer_var = 1;
    let closure_1 = |input_var: u32| -> u32 { input_var + outer_var };
    println!("Closure#1 returns {}", closure_1(5));

    let closure_2 = |input_var| println!("Closure#2 (input_var {})", input_var);
    closure_2(2);
    closure_2(5);
}
