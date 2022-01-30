fn palindrome(arr: [i32; 7], len: usize) -> bool {
    // write down the algorithm here

    true
}

fn main() {
    let arr1 = [1, 0, 2, 3, 2, 2, 1];
    let arr2 = [2, 0, 2, 3, 2, 0, 2];
    assert_eq!(palindrome(arr1, arr1.len()), false);
    assert_eq!(palindrome(arr2, arr2.len()), true);
}
