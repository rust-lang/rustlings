fn bubbleSort(mut arr: [i32; 7], len: usize) -> [i32; 7] {
    // write down the algorithm here

    arr
}

fn printArray(arr: [i32; 7], len: usize) {
    for item in arr.iter() {
        println!("{}", item);
    }
}

fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    let sort = [11, 12, 22, 25, 34, 64, 90];
    arr = bubbleSort(arr, arr.len());
    println!("Sorted array: ");
    printArray(arr, arr.len());
    assert_eq!(arr, sort);
}
