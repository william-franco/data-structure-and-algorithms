//! Iterative Binary Search
//!
//! Performs binary search on a sorted `Vec<i32>`, returning `Option<usize>`.

fn binary_search_iter(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let data = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
    assert_eq!(binary_search_iter(&data, 23), Some(5));
    assert_eq!(binary_search_iter(&data, 100), None);
    println!("Found at index: {:?}", binary_search_iter(&data, 12));
}
