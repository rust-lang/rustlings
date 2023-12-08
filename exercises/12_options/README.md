# Options

Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not.
Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error
- Optional struct fields
- Struct fields that can be loaned or "taken"
- Optional function arguments
- Nullable pointers
- Swapping things out of difficult situations

## Further Information

- [Option Enum Format](https://rust-book.cs.brown.edu/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://rust-book.cs.brown.edu/std/option/)
- [Option Enum Documentation](https://rust-book.cs.brown.edu/std/option/enum.Option.html)
- [if let](https://rust-book.cs.brown.edu/rust-by-example/flow_control/if_let.html)
- [while let](https://rust-book.cs.brown.edu/rust-by-example/flow_control/while_let.html)
