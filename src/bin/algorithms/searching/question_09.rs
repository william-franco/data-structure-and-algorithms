//! Exponential Search
//!
//! Finds a value in a sorted `Vec<i32>` using exponential search.

fn exponential_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    if arr[0] == target {
        return Some(0);
    }

    let mut bound = 1;
    while bound < arr.len() && arr[bound] <= target {
        bound *= 2;
    }

    let left = bound / 2;
    let right = bound.min(arr.len() - 1);
    binary_search_range(arr, target, left, right)
}

fn binary_search_range(arr: &[i32], target: i32, mut left: usize, mut right: usize) -> Option<usize> {
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else if mid == 0 {
            return None;
        } else {
            right = mid - 1;
        }
    }
    None
}

fn main() {
    let data: Vec<i32> = (1..=1000).collect();
    assert_eq!(exponential_search(&data, 1), Some(0));
    assert_eq!(exponential_search(&data, 512), Some(511));
    assert_eq!(exponential_search(&data, 1001), None);
    println!("Found: {:?}", exponential_search(&data, 777));
}
