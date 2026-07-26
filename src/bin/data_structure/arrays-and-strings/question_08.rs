//! Merge Sorted Arrays
//!
//! Merges two sorted `Vec<i32>` into a single sorted vector without using `.sort()`.

fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

fn main() {
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];
    assert_eq!(merge_sorted(&a, &b), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    println!("Merged: {:?}", merge_sorted(&a, &b));
}
