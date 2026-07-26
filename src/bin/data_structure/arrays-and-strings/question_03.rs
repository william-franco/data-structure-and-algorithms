//! Anagram Check
//!
//! Checks if two strings are anagrams of each other (same letters, possibly different order).

fn are_anagrams(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut count = [0i32; 256];
    for byte in s1.bytes() {
        count[byte as usize] += 1;
    }
    for byte in s2.bytes() {
        count[byte as usize] -= 1;
        if count[byte as usize] < 0 {
            return false;
        }
    }
    true
}

fn main() {
    assert!(are_anagrams("listen", "silent"));
    assert!(!are_anagrams("hello", "world"));
    println!("Anagrams: {}", are_anagrams("triangle", "integral"));
}
