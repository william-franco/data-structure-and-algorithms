//! Subset Sum
//!
//! Uses backtracking to check if any subset of a `Vec<i32>` sums to a target value.

fn subset_sum(arr: &[i32], target: i32) -> bool {
    backtrack(arr, 0, target)
}

fn backtrack(arr: &[i32], index: usize, remaining: i32) -> bool {
    if remaining == 0 {
        return true;
    }
    if index >= arr.len() || remaining < 0 {
        return false;
    }
    backtrack(arr, index + 1, remaining - arr[index]) || backtrack(arr, index + 1, remaining)
}

fn main() {
    let data = vec![3, 34, 4, 12, 5, 2];
    assert!(subset_sum(&data, 9));
    assert!(!subset_sum(&data, 30));
    println!("Sum 9 exists: {}", subset_sum(&data, 9));
}
