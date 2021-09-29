// fizzbuzz challenge

// Make a simple code which fulfills the given tests
// Starting code is already given to you :)

/* Write a program that substitues the following:
 * mulitples of 3 => "fizz"
 * multiples of 5 => "buzz"
 * mulitples of 3 and 5 => "fizzbuzz"
 */

// I AM NOT DONE

pub fn fizzbuzz(num: i32) -> String {
}

// Don't change this part(the tests)

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn fizz() {
		assert_eq!(fizzbuzz(3), "fizz".to_string());
		assert_eq!(fizzbuzz(6), "fizz".to_string());
		assert_eq!(fizzbuzz(53), "53".to_string());
	}

	#[test]
	fn buzz() {
		assert_eq!(fizzbuzz(5), "buzz".to_string());
		assert_eq!(fizzbuzz(10), "buzz".to_string());
		assert_eq!(fizzbuzz(13), "13".to_string());
	}

	#[test]
	fn fizz_and_buzz() {
		assert_eq!(fizzbuzz(15), "fizzbuzz".to_string());
		assert_eq!(fizzbuzz(45), "fizzbuzz".to_string());
		assert_eq!(fizzbuzz(31), "31".to_string());
	}
}
