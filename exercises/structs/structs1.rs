// structs1.rs
// Address all the TODOs to make the tests pass!

// note May 29/21 the solution for this takes more time than usual
// DONE

struct ColorClassicStruct {
    // TODO: Something goes here
	color: String,
	hex: String,
}

//struct ColorTupleStruct(/* TODO: Something goes here */);
struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct {
			color: String::from("green"),
			hex: String::from("#00FF00"),
		};

        assert_eq!(green.color, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let UnitStruct;
        let message = format!("{:?}s are fun!", UnitStruct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
