//! First and Last Occurrence
//!
//! Finds the first and last index of a value in a sorted `Vec<i32>` with duplicates
//! using binary search.

fn find_first(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    let mut result = None;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    result
}

fn find_last(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    let mut result = None;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            result = Some(mid);
            left = mid + 1;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    result
}

fn find_range(arr: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
    (find_first(arr, target), find_last(arr, target))
}

fn main() {
    let data = vec![1, 2, 2, 2, 3, 4, 4, 5];
    assert_eq!(find_range(&data, 2), (Some(1), Some(3)));
    assert_eq!(find_range(&data, 4), (Some(5), Some(6)));
    assert_eq!(find_range(&data, 6), (None, None));
    println!("Range for 2: {:?}", find_range(&data, 2));
}
