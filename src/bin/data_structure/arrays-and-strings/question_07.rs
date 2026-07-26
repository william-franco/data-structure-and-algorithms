//! Unique Characters
//!
//! Checks if a string has all unique characters without auxiliary data structures.

fn has_unique_chars(s: &str) -> bool {
    let bytes: Vec<u8> = s.bytes().collect();
    for i in 0..bytes.len() {
        for j in i + 1..bytes.len() {
            if bytes[i] == bytes[j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    assert!(has_unique_chars("abcdef"));
    assert!(!has_unique_chars("hello"));
    println!("Unique 'rust': {}", has_unique_chars("rust"));
}
