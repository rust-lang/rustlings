// structs4.rs
// Structs can have methods and the first parameter is always self. In this exercise
// we have defined the Planet struct and we want to test some logic attached to it,
// make the code compile and the tests pass! If you have issues execute `rustlings hint structs4`

// I AM NOT DONE

#[derive(Debug)]
struct Planet {
    has_life: bool,
    radius : u32
}

impl Planet {
    fn new(radius: u32) -> Planet {
        // Something goes here...
    }

    fn has_life(???) -> bool {
        // Something goes here...
    }

    fn change_radius(???) {
        // Something goes here...
    }

    fn create_life(???) {
        // Something goes here... 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_planet() {

        let planet = Planet::new(1000);

        assert_eq!(planet.radius, 1000);
        assert_eq!(planet.has_life(), false);
    }

    #[test]
    fn add_life_to_planet() {
        let planet = Planet::new(1000);

        planet.create_life();
        
        assert_eq!(planet.has_life(), true);
    }

    #[test]
    fn change_radius_of_planet() {
        let mut planet = Planet::new(1000);

        planet.change_radius(2000);
        
        assert_eq!(planet.radius, 2000);
    }
}
