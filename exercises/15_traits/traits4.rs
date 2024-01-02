// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE

// 1. straight foreword
// fn compare_license_types(software: impl Licensed, software_two: impl Licensed) -> bool {
//     software.licensing_info() == software_two.licensing_info()
// }

// 2. trait bound
// fn compare_license_types<T: Licensed, P: Licensed>(software: T, software_two: P) -> bool {
//     software.licensing_info() == software_two.licensing_info()
// }


// 3. trait bound + where clause
fn compare_license_types<T, P>(software: T, software_two: P) -> bool 
    where T: Licensed,
          P: Licensed
{
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
