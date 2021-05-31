// structs2.rs
// Address all the TODOs to make the tests pass!

// Not Done

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

// let your_order = Order{ 
// 	name: String::from("Hacker in Rust"),
// 	count: 1,
// 	..order_template	//dot dot operator meaning the rest of struct fields remains the same
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!

        // this struct is moved from above to inside this function.
		// create own struct version changing the name and count while the remaining fields remain the same
		let your_order = Order{ 
			name: String::from("Hacker in Rust"),
			count: 1,
			..order_template	//dot dot operator meaning the rest of struct fields remain the same
		};
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
        //assert_eq!(your_order.count, 0);
    }
}
