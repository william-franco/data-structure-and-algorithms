//! Power Set (Subsets)
//!
//! Generates all possible subsets of a `Vec<i32>` using recursion.

fn subsets(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    generate_subsets(arr, 0, &mut current, &mut result);
    result
}

fn generate_subsets(
    arr: &[i32],
    index: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if index == arr.len() {
        result.push(current.clone());
        return;
    }
    generate_subsets(arr, index + 1, current, result);
    current.push(arr[index]);
    generate_subsets(arr, index + 1, current, result);
    current.pop();
}

fn main() {
    let data = vec![1, 2, 3];
    let result = subsets(&data);
    assert_eq!(result.len(), 8);
    println!("Subsets: {:?}", result);
}
