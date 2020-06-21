// structs3.rs
// Structs contain more than simply some data, they can also have logic, in this
// exercise we have defined the Package struct and we want to test some logic attached to it,
// make the code compile and the tests pass! If you have issues execute `rustlings hint structs3`

#[derive(Debug)]
struct Package {
    from: String,
    to: String,
    weight: f32,
}

impl Package {
    fn new(from: String, to: String, weight: f32) -> Package {
        if weight <= 0.0 {
            let msg = format!("Weight must be greater than 0, {} specified", weight);
            panic!(msg)
        } else {
            return Package { from, to, weight };
        }
    }

    fn is_international(&self) -> bool {
        if self.from == self.to {
            false
        } else {
            true
        }
    }

    fn get_fees(&self, cost_per_kg: f32) -> f32 {
        self.weight * cost_per_kg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let country_from = String::from("Spain");
        let country_to = String::from("Austria");

        Package::new(country_from, country_to, -2.21);
    }

    #[test]
    fn create_international_package() {
        let country_from = String::from("Spain");
        let country_to = String::from("Russia");
        let package = Package::new(country_from, country_to, 1.2);

        assert!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let country_from = String::from("Spain");
        let country_to = String::from("Spain");

        let country_fee = 8.;
        let package = Package::new(country_from, country_to, 22.0);
        assert_eq!(package.get_fees(country_fee), 176.0);
    }
}
