//! Quick Sort
//!
//! Sorts a `Vec<i32>` in ascending order using the quick sort algorithm.

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    let mut data = vec![10, 7, 8, 9, 1, 5];
    quick_sort(&mut data);
    assert_eq!(data, vec![1, 5, 7, 8, 9, 10]);
    println!("Sorted: {:?}", data);
}
