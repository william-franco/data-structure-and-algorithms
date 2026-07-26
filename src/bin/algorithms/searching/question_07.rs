//! Binary Search in Rotated Array
//!
//! Finds a value in a sorted array that has been rotated at an unknown pivot.

fn search_rotated(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        }
        if arr[left] <= arr[mid] {
            if arr[left] <= target && target < arr[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        } else if arr[mid] < target && target <= arr[right - 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let data = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(search_rotated(&data, 0), Some(4));
    assert_eq!(search_rotated(&data, 3), None);
    println!("Found: {:?}", search_rotated(&data, 7));
}
