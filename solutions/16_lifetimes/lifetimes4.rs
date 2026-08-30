// Sometimes, we have structs which hold on to data temporarily. A use-case of
// this could be a routing component which accepts a connection and returns it to
// another recipient. To avoid copying the data, we just accept a reference with
// lifetime and return this reference later.
//
// In the example below, we create a `Router` instance in a limited scope. It
// accepts a connection reference created in the enclosing scope and returns it.
// In theory, this should be possible given that the connection reference outlives
// the scope from which it is returned. However, the borrow checker does not
// seem to understand it. What can we do about that?

struct Router<'a> {
    connection: Option<&'a u64>,
    //                  ^^ lifetime of the connection reference
    //                     which may outlive the `Router` itself
}

impl<'a> Router<'a> {
    fn new() -> Self {
        Self { connection: None }
    }

    fn accept_connection(&mut self, connection: &'a u64) {
        self.connection = Some(connection);
    }

    fn return_connection(&mut self) -> Option<&'a u64> {
        //           added lifetime annotation ^^
        //
        //           Without annotation, the compiler infers the output reference
        //           to have the lifetime of the only input reference
        //           -> the lifetime of `&mut self`.
        self.connection.take()
    }
}

fn main() {
    let connection = &123;

    let returned_connection = {
        // Create router within scope
        let mut router = Router::new();

        // Accept connection which lives longer than the router
        router.accept_connection(connection);

        // Return connection which **should** live longer than the router
        router.return_connection()
        //    ^^^^^^^^^^^^^^^^^^^^
        //
        //    Without the explicit lifetime annotation in `return_connection`,
        //    the reference from `return_connection` has the lifetime of `router`.
        //    We are returning the reference from the scope, requiring it to outlive it,
        //    so the compiler complains about `router` not living long enough.
    };

    if let Some(connection) = returned_connection {
        println!("The connection is {connection}");
    }
}
