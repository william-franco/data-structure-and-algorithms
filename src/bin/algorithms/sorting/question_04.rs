//! Merge Sort
//!
//! Sorts a `Vec<i32>` in ascending order using the divide-and-conquer approach.

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    merge(arr, mid);
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut data = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut data);
    assert_eq!(data, vec![3, 9, 10, 27, 38, 43, 82]);
    println!("Sorted: {:?}", data);
}
