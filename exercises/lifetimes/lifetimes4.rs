// lifetimes4.rs
//
// Sometimes, we have structs which hold on to data temporarily. A use-case of
// this could be a routing component which accepts data and returns it to
// another recipient. To avoid copying the data, we just accept a reference with
// lifetime and return this reference later.
//
// In the example below, we create a `Router` instance in a limited scope. It
// accepts a number reference created in the enclosing scope and returns it.
// In theory, this should be possible given that the number reference outlives
// the scope from which it is returned. However the borrow checker does not
// seem to understand it. What can we do about that?
//
// Execute `rustlings hint lifetimes4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Router<'a> {
    number_ref: Option<&'a u64>,
}

impl<'a> Router<'a> {
    fn new() -> Self {
        Self { number_ref: None }
    }

    fn take_number_ref(&mut self, number_ref: &'a u64) {
        self.number_ref = Some(number_ref);
    }

    fn return_number_ref(&mut self) -> Option<&u64> {
        self.number_ref.take().take()
    }
}

fn main() {
    let number_ref = &123;

    let returned_ref = {
        // Create router within scope.
        let mut router = Router::new();

        // Accept number ref which lives longer than the router.
        router.take_number_ref(number_ref);

        // Return number ref which **should** live longer than the router.
        router.return_number_ref()
    };

    if let Some(number) = returned_ref {
        println!("The number is {number}");
    }
}
