//! Two Sum
//!
//! Returns indices of two elements in a `Vec<i32>` whose sum equals the target.

fn two_sum(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] + arr[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let data = vec![2, 7, 11, 15];
    assert_eq!(two_sum(&data, 9), Some((0, 1)));
    assert_eq!(two_sum(&data, 100), None);
    println!("Two sum indices: {:?}", two_sum(&data, 9));
}
