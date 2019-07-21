// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}

struct ColorTupleStruct<'a>(&'a str, &'a str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let name = "green";
        let hex = "#00FF00";
        let green = ColorClassicStruct { name, hex };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let name = "green";
        let hex = "#00FF00";
        let green = ColorTupleStruct(name, hex);
        // For more fun, use the field initialization shorthand.
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
