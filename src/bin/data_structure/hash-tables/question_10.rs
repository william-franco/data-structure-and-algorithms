//! Detect Duplicates
//!
//! Detects if a `Vec<i32>` contains duplicate elements using `HashSet` in O(n).

use std::collections::HashSet;

fn tem_duplicatas(arr: &[i32]) -> bool {
    let mut vistos = HashSet::new();
    for &val in arr {
        if !vistos.insert(val) {
            return true;
        }
    }
    false
}

fn main() {
    assert!(tem_duplicatas(&[1, 2, 3, 1]));
    assert!(!tem_duplicatas(&[1, 2, 3, 4]));
    println!("Has duplicates: {}", tem_duplicatas(&[1, 2, 3, 1]));
}
