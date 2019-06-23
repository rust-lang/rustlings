// errorsn.rs
// This is a bigger error exercise than the previous ones!
// You can do it! :)
//
// Edit the `read_and_validate` function so that it compiles and
// passes the tests... so many things could go wrong!
//
// - Reading from stdin could produce an io::Error
// - Parsing the input could produce a num::ParseIntError
// - Validating the input could produce a CreationError (defined below)
//
// How can we lump these errors into one general error? That is, what
// type goes where the question marks are, and how do we return
// that type from the body of read_and_validate?
//
// Scroll down for hints :)

use std::error;
use std::fmt;
use std::io;

// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(b: &mut io::BufRead) -> Result<PositiveNonzeroInteger, ???> {
    let mut line = String::new();
    b.read_line(&mut line);
    let num: i64 = line.trim().parse();
    let answer = PositiveNonzeroInteger::new(num);
    answer
}

// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42\n");
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
    assert_eq!("uh-oh!", read_and_validate(&mut b).unwrap_err().to_string());
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &error::Error).description())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}































// First hint: To figure out what type should go where the ??? is, take a look
// at the test helper function `test_with_str`, since it returns whatever
// `read_and_validate` returns and`test_with_str` has its signature fully
// specified.




















// Next hint: There are three places in `read_and_validate` that we call a
// function that returns a `Result` (that is, the functions might fail).
// Apply the `?` operator on those calls so that we return immediately from
// `read_and_validate` if those function calls fail.




















// Another hint: under the hood, the `?` operator calls `From::from`
// on the error value to convert it to a boxed trait object, a Box<error::Error>,
// which is polymorphic-- that means that lots of different kinds of errors
// can be returned from the same function because all errors act the same
// since they all implement the `error::Error` trait.
// Check out this section of the book:
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator




















// Another another hint: Note that because the `?` operator returns
// the *unwrapped* value in the `Ok` case, if we want to return a `Result` from
// `read_and_validate` for *its* success case, we'll have to rewrap a value
// that we got from the return value of a `?`ed call in an `Ok`-- this will
// look like `Ok(something)`.




















// Another another another hint: `Result`s must be "used", that is, you'll
// get a warning if you don't handle a `Result` that you get in your
// function. Read more about that in the `std::result` module docs:
// https://doc.rust-lang.org/std/result/#results-must-be-used
