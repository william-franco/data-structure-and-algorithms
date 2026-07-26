//! Combinations
//!
//! Generates all combinations of K elements from a `Vec<i32>` of N elements.

fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    combine(arr, k, 0, &mut current, &mut result);
    result
}

fn combine(
    arr: &[i32],
    k: usize,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    for i in start..arr.len() {
        current.push(arr[i]);
        combine(arr, k, i + 1, current, result);
        current.pop();
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let result = combinations(&data, 2);
    assert_eq!(result.len(), 6);
    println!("Combinations: {:?}", result);
}
