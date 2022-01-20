// structs1.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE


#[derive(Debug, PartialEq)]
struct ColorClassicStruct {
    name : String,
    hex : String
}

struct ColorTupleStruct(String,String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct{ name : "green".to_string(), hex :"#00FF00".to_string()};
        // ### since String != strliteral but rather String < strliteral (??) we need to use
        // to_string()

        assert_eq!(green,green,"probando");
        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct("green".to_string(),"#00FF00".to_string());

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
