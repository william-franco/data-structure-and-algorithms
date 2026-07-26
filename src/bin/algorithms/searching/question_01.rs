//! Linear Search
//!
//! Searches for a value in a `Vec<i32>` and returns `Option<usize>` with the found index.

fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &val) in arr.iter().enumerate() {
        if val == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let data = vec![10, 20, 80, 30, 60, 50, 110];
    assert_eq!(linear_search(&data, 30), Some(3));
    assert_eq!(linear_search(&data, 100), None);
    println!("Found at index: {:?}", linear_search(&data, 60));
}
