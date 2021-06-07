// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

// I AM NOT DONE

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    CreationError,
    ParseIntError
}

fn parse_pos_nonzero(s: &str)
    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().unwrap();
    PositiveNonzeroInteger::new(x)
        .or(Err(ParsePosNonzeroError::CreationError))
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert_eq!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseIntError)
        );
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::CreationError)
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::CreationError)
        );
    }

    #[test]
    fn test_positive() {
        assert_eq!(
            parse_pos_nonzero("42"),
            Ok(PositiveNonzeroInteger(42))
        );
    }
}
