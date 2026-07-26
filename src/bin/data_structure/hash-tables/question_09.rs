//! Longest Subarray with Zero Sum
//!
//! Finds the length of the longest contiguous subarray with sum zero using `HashMap`.

use std::collections::HashMap;

fn maior_subarray_soma_zero(arr: &[i32]) -> usize {
    let mut soma = 0;
    let mut mapa: HashMap<i32, i32> = HashMap::new();
    mapa.insert(0, -1);
    let mut max_len: usize = 0;

    for (i, &val) in arr.iter().enumerate() {
        soma += val;
        if let Some(&j) = mapa.get(&soma) {
            max_len = max_len.max(i - j as usize);
        } else {
            mapa.insert(soma, i as i32);
        }
    }
    max_len
}

fn main() {
    assert_eq!(maior_subarray_soma_zero(&[15, -2, 2, -8, 1, 7, 10, 23]), 5);
    assert_eq!(maior_subarray_soma_zero(&[1, 2, 3]), 0);
    println!(
        "Longest zero-sum subarray: {}",
        maior_subarray_soma_zero(&[15, -2, 2, -8, 1, 7, 10, 23])
    );
}
