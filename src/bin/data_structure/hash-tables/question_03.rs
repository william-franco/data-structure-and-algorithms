//! Anagram Check with HashMap
//!
//! Checks if two strings are anagrams using `HashMap<char, i32>` for character frequency.

use std::collections::HashMap;

fn sao_anagramas(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut freq: HashMap<char, i32> = HashMap::new();
    for ch in s1.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    for ch in s2.chars() {
        let count = freq.entry(ch).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }
    true
}

fn main() {
    assert!(sao_anagramas("listen", "silent"));
    assert!(!sao_anagramas("hello", "world"));
    println!("Anagrams: {}", sao_anagramas("triangle", "integral"));
}
