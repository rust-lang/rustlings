// traits6.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits6` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct House {
    area_sqft: u32,
    purchase_date: String,
}

struct Vehicle {
    name: String,
    model: String,
    purchase_date: String,
}

trait Details {
    fn summary(&self) -> String;
}

impl Details for House {
    fn summary(&self) -> String {
        format!(
            "The house has an area of {} sqft and was purchased on {}",
            self.area_sqft, self.purchase_date
        )
    }
}
impl Details for Vehicle {
    fn summary(&self) -> String {
        format!(
            "The {} vehicle with model {} was purchased on {}",
            self.name, self.model, self.purchase_date
        )
    }
}

// TODO: Complete the code
fn foo(flag: bool) -> ?? {
    if flag {
        Box::new(House {
            area_sqft: 5000,
            purchase_date: "21 Nov 2017".to_string(),
        })
    } else {
        Box::new(Vehicle {
            name: "BMW".to_string(),
            model: "320d".to_string(),
            purchase_date: "13 Aug 2022".to_string(),
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn init() -> (House, Vehicle) {
        let house = House {
            area_sqft: 5000,
            purchase_date: "21 Nov 2017".to_string(),
        };
        let vehicle = Vehicle {
            name: "BMW".to_string(),
            model: "320d".to_string(),
            purchase_date: "13 Aug 2022".to_string(),
        };

        (house, vehicle)
    }

    #[test]
    fn check_foo_returns_house_if_true() {
        let (house, _) = init();
        assert_eq!(house.summary(), foo(true).summary());
    }

    #[test]
    fn check_foo_returns_vehicle_if_false() {
        let (_, vehicle) = init();
        assert_eq!(vehicle.summary(), foo(false).summary());
    }

    #[test]
    fn check_purchase_date_for_house() {
        let (house, _) = init();
        assert_eq!(
            format!(
                "The house has an area of {} sqft and was purchased on {}",
                house.area_sqft, house.purchase_date
            ),
            house.summary()
        );
    }

    #[test]
    fn check_purchase_date_for_vehicle() {
        let (_, vehicle) = init();
        assert_eq!(
            format!(
                "The {} vehicle with model {} was purchased on {}",
                vehicle.name, vehicle.model, vehicle.purchase_date
            ),
            vehicle.summary()
        );
    }
}
