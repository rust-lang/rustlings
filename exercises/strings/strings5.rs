// strings5.rs
// println!, print!, and format! use the same system for created formatted text.
// Make all tests pass by only changing the first parameter to each format! call.
// Execute `rustlings hint strings5` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[cfg(test)]
mod tests {

    //TODO only change first parameter in each format! call.
    #[test]
    fn greeting() {
        let world = "world";
        //The basic usage you probably saw used in println! statements
        assert_eq!(format!("Hello, !", world), "Hello, world!");

        //There is a way to get the same result inline, without passing world as an argument.
        assert_eq!(format!("Hello, {}!"), "Hello, world!");
    }

    #[test]
    fn bond() {
        //You can use the same argument multiple times,
        //as well as specicfy which argument should be used for a given {} placeholder.
        assert_eq!(format!("My name is {}. {} {}.", "James", "Bond"), "My name is Bond. James Bond.");
        //There are also ways to specify how numbers should be displayed.
        assert_eq!(format!("My code number is {}.", 7), "My code number is 007.");
    }
}
