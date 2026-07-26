//! First Non-Repeating Element
//!
//! Finds the first non-repeating element in a `Vec<i32>` using `HashMap` for counting.

use std::collections::HashMap;

fn primeiro_nao_repetido(arr: &[i32]) -> Option<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &val in arr {
        *freq.entry(val).or_insert(0) += 1;
    }
    for &val in arr {
        if freq[&val] == 1 {
            return Some(val);
        }
    }
    None
}

fn main() {
    assert_eq!(primeiro_nao_repetido(&[1, 2, 2, 3, 3, 4, 4]), Some(1));
    assert_eq!(primeiro_nao_repetido(&[2, 2, 3, 3]), None);
    println!("First unique: {:?}", primeiro_nao_repetido(&[1, 2, 2, 3, 3, 4, 4]));
}
