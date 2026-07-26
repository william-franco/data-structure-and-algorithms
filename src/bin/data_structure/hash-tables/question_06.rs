//! Two Sum (HashMap Optimized)
//!
//! Finds two indices whose values sum to target in O(n) using `HashMap<i32, usize>`.

use std::collections::HashMap;

fn two_sum(arr: &[i32], alvo: i32) -> Option<(usize, usize)> {
    let mut mapa: HashMap<i32, usize> = HashMap::new();
    for (i, &val) in arr.iter().enumerate() {
        let complemento = alvo - val;
        if let Some(&j) = mapa.get(&complemento) {
            return Some((j, i));
        }
        mapa.insert(val, i);
    }
    None
}

fn main() {
    let data = vec![2, 7, 11, 15];
    assert_eq!(two_sum(&data, 9), Some((0, 1)));
    assert_eq!(two_sum(&data, 100), None);
    println!("Two sum indices: {:?}", two_sum(&data, 9));
}
