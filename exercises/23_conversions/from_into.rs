// The `From` trait is used for value-to-value conversions. If `From` is
// implemented, an implementation of `Into` is automatically provided.
// You can read more about it in the documentation:
// https://doc.rust-lang.org/std/convert/trait.From.html

use std::convert::{TryFrom, TryInto};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// We will use this error type for the `TryFrom` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO: Complete this `TryFrom` implementation to be able to parse a `Person`
// out of a string in the form of "Mark,20".
// Note that you'll need to parse the age component into a `u8` with something
// like `"4".parse::<u8>()`.
//
// Steps:
// 1. Split the given string on the commas present in it.
// 2. If the split operation returns less or more than 2 elements, return the
//    error `ParsePersonError::BadLen`.
// 3. Use the first element from the split operation as the name.
// 4. If the name is empty, return the error `ParsePersonError::NoName`.
// 5. Parse the second element from the split operation into a `u8` as the age.
// 6. If parsing the age fails, return the error `ParsePersonError::ParseInt`.
impl TryFrom<&str> for Person {
    type Error = ParsePersonError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {}
}

fn main() {
    // Use the `try_from` function.
    let p1 = Person::try_from("Mark,20");
    println!("{p1:?}");

    // Since `TryFrom` is implemented for Person, we are able to use `try_into`.
    let p2: Result<Person, ParsePersonError> = "Gerald,70".try_into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!(Person::try_from(""), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = Person::try_from("John,32");
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!(Person::try_from("John,"), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(Person::try_from("John,twenty"), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!(Person::try_from("John"), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(Person::try_from(",1"), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(Person::try_from(","), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            Person::try_from(",one"),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!(Person::try_from("John,32,"), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(Person::try_from("John,32,man"), Err(BadLen));
    }
}
