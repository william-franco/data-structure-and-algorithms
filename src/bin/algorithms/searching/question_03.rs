//! Recursive Binary Search
//!
//! Performs recursive binary search on a sorted slice, returning `Option<usize>`.

fn binary_search_rec(arr: &[i32], target: i32) -> Option<usize> {
    binary_search_helper(arr, target, 0, arr.len())
}

fn binary_search_helper(arr: &[i32], target: i32, left: usize, right: usize) -> Option<usize> {
    if left >= right {
        return None;
    }
    let mid = left + (right - left) / 2;
    if arr[mid] == target {
        Some(mid)
    } else if arr[mid] < target {
        binary_search_helper(arr, target, mid + 1, right)
    } else {
        binary_search_helper(arr, target, left, mid)
    }
}

fn main() {
    let data = vec![1, 3, 5, 7, 9, 11, 13];
    assert_eq!(binary_search_rec(&data, 7), Some(3));
    assert_eq!(binary_search_rec(&data, 4), None);
    println!("Found at index: {:?}", binary_search_rec(&data, 11));
}
