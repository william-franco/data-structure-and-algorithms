//! Selection Sort
//!
//! Sorts a `Vec<i32>` in ascending order without using the standard `.sort()` method.

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

fn main() {
    let mut data = vec![29, 10, 14, 37, 13];
    selection_sort(&mut data);
    assert_eq!(data, vec![10, 13, 14, 29, 37]);
    println!("Sorted: {:?}", data);
}
