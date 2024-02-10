// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let new_str = String::from(input);
    let output_str: String = String::from("");
    let mut count = 0;
    for (i, &item) in new_str.as_bytes().iter().enumerate() {
        if item == b' ' {
            count += 1;
            continue;
        }
        break;
    }
    let trimmed_start: &str = &new_str[count..];

    let mut count = 0;
    for i in trimmed_start.chars() {
        if i == ' ' {
            count += 1;
            continue;
        }
    }
    let final_str = &trimmed_start[..trimmed_start.len() - count];
    String::from(final_str)
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut output = String::from(input);
    output.push_str(" world!");
    output
}

fn replace_me(input: &str) -> String {
    let some_str = String::from(input);
    let x = some_str.replace("cars", "balloons");
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
