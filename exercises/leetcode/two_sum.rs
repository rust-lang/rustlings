// TODO: Implement the "Two Sum" problem in Rust.
// Given an array of integers `nums` and an integer `target`,
// return the indices of the two numbers such that they add up to `target`.
//
// Hint: Try using a HashMap for O(n) time.
// Remember ownership & borrowing rules for inserting into the map.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
