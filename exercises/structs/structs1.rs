// structs1.rs
// Address all the TODOs to make the tests pass!


#[derive(Debug)]
struct ColorClassicStruct {
    // TODO: Something goes here
    name:String,
    hex:String,
}


struct ColorTupleStruct();

#[derive(Debug)]
struct UnitStruct{
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct{name:String::from("green"),
                                        hex:String::from("#00FF00")};

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green =(String::from("green"),String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_structs:UnitStruct=UnitStruct{};
        let message = format!("{:?}s are fun!", unit_structs);

        assert_eq!(message, "UnitStructs are fun!");
    }
}


