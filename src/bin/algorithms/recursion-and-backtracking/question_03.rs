//! Permutations
//!
//! Generates all permutations of the elements in a `Vec<i32>` using backtracking.

fn permutations(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; arr.len()];
    permute(arr, &mut current, &mut used, &mut result);
    result
}

fn permute(
    arr: &[i32],
    current: &mut Vec<i32>,
    used: &mut [bool],
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == arr.len() {
        result.push(current.clone());
        return;
    }
    for i in 0..arr.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        current.push(arr[i]);
        permute(arr, current, used, result);
        current.pop();
        used[i] = false;
    }
}

fn main() {
    let data = vec![1, 2, 3];
    let result = permutations(&data);
    assert_eq!(result.len(), 6);
    println!("Permutations: {:?}", result);
}
