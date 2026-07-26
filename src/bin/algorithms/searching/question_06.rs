//! Rotation Point in Sorted Array
//!
//! Finds the index of the minimum element (rotation point) in a rotated sorted array.

fn find_rotation_point(arr: &[i32]) -> usize {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] > arr[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn main() {
    let data = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(find_rotation_point(&data), 4);
    let data2 = vec![1, 2, 3, 4, 5];
    assert_eq!(find_rotation_point(&data2), 0);
    println!("Rotation point: {}", find_rotation_point(&data));
}
