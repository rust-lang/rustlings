// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return the default of Person
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person object with the results

//impl From<&str> for Person {
//    fn from(s: &str) -> Person {
//        if s.len() == 0 {
//            return Person::default();
//        }
//        let string_vector = s.split(',').collect::<Vec<&str>>();
//        println!("{:?}", string_vector);
//        if string_vector[0] == "" || string_vector.len() < 2 {
//            return Person::default();
//        } else {
//            match string_vector[1].parse::<usize>() {
//                Ok(result) => Person {
//                    name: string_vector[0].to_string(),
//                    age: result,
//                },
//                Err(_) => Person::default(),
//            }
//        }
//    }
//}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // splits just in 2 parts
        let parts = s.splitn(2, ',').collect::<Vec<&str>>();
        // match all parts
        match &parts[..] {
            [name, age] if !name.is_empty() => age
                .parse()
                .map(|age| Person {
                    name: name.to_string(),
                    age,
                })
                .unwrap_or_default(),
            _ => Person::default(),
        }
    }
}

//impl From<&str> for Person {
//    fn from(s: &str) -> Person {
//        if s.len() == 0 {
//            Person::default()
//        } else {
//            let mut person_itr = s.split(',');
//            match person_itr.next().map(|name| {
//                (
//                    if name.is_empty() {
//                        Err("Missing Name")
//                    } else {
//                        Ok(name.to_string())
//                    },
//                    person_itr.collect::<String>().parse::<usize>(),
//                )
//            }) {
//                Some((Ok(name), Ok(age))) => Person { name, age },
//                _ => Person::default(),
//            }
//        }
//    }
//}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
