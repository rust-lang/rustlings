fn main() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("counter equals {}", counter);
    };

    increment();
    increment();
    let _reborrowed_counter = &counter;

    assert_eq!(counter, 2);
}
