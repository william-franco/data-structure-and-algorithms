//! Maximum Subarray Sum (Kadane's Algorithm)
//!
//! Finds the maximum sum of a contiguous subarray using Kadane's algorithm.

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];

    for &val in &arr[1..] {
        max_ending_here = val.max(max_ending_here + val);
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}

fn main() {
    assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_subarray_sum(&[1]), 1);
    println!("Max subarray sum: {}", max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}
