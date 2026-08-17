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

## Conditional pattern matching

`if let` runs a block once when a value matches a pattern:

```rust
if let PATTERN = EXPRESSION {
    // The pattern matched.
}
```

`while let` repeats a block for as long as the value matches a pattern:

```rust
while let PATTERN = EXPRESSION {
    // The pattern matched. Try the expression again after this iteration.
}
```

The left side of `=` is a pattern, while the right side is the expression whose
result is matched. These constructs are useful when only one pattern matters.
Patterns can also be nested to match nested types such as `Option<Option<T>>`.

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
