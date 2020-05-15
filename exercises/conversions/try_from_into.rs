// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// I AM NOT DONE

// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need create implementation for a tuple of three integer,
// an array of three integer and slice of integer.
//
// Note, that implementation for tuple and array will be checked at compile-time,
// but slice implementation need check slice length!
// Also note, that chunk of correct rgb color must be integer in range 0..=255.

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
    }
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_positive() {
        let _ = Color::try_from((256, 1000, 10000)).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_negative() {
        let _ = Color::try_from((-1, -10, -256)).unwrap();
    }
    #[test]
    fn test_tuple_correct() {
        let c: Color = (183, 65, 14).try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_array_out_of_range_positive() {
        let _: Color = [1000, 10000, 256].try_into().unwrap();
    }
    #[test]
    #[should_panic]
    fn test_array_out_of_range_negative() {
        let _: Color = [-10, -256, -1].try_into().unwrap();
    }
    #[test]
    fn test_array_correct() {
        let c: Color = [183, 65, 14].try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        let _ = Color::try_from(&arr[..]).unwrap();
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c = Color::try_from(&v[..]).unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }
    #[test]
    #[should_panic]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        let _ = Color::try_from(&v[..]).unwrap();
    }
}
