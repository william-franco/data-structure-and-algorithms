//! Frequency Count
//!
//! Counts the frequency of each element in a `Vec<i32>` using `HashMap<i32, i32>`.

use std::collections::HashMap;

fn contar_frequencia(arr: &[i32]) -> HashMap<i32, i32> {
    let mut freq = HashMap::new();
    for &val in arr {
        *freq.entry(val).or_insert(0) += 1;
    }
    freq
}

fn main() {
    let data = vec![1, 2, 2, 3, 3, 3, 4];
    let freq = contar_frequencia(&data);
    assert_eq!(freq[&1], 1);
    assert_eq!(freq[&2], 2);
    assert_eq!(freq[&3], 3);
    println!("Frequency: {:?}", freq);
}
