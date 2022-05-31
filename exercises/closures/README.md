# Closures

Closures are a common programming concept found in many languages, but are primarily
associated with 'functional' programming styles. Closure are anonymous functions
(i.e. unlike regular functions, they don't have a name) that may access some variables
from their enclosing scope (they 'close' over their scope, capturing variables).

Closures are often used as arguments to higher order functions (functions which 
take other functions as parameters), e.g. map(), filter(), reduce() et al on iterators,
but aren't restricted to these cases.

Rust's ownership model means that we may need to think about how captured values
are treated: copied, moved, or borrowed immutably or mutably. 

## Further information

- [Closures: Anonymous Functions that Can Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html)