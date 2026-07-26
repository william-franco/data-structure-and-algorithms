//! Insertion Sort
//!
//! Sorts a `Vec<i32>` in ascending order without using the standard `.sort()` method.

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut data = vec![5, 2, 4, 6, 1, 3];
    insertion_sort(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    println!("Sorted: {:?}", data);
}
