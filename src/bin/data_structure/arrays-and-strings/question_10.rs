//! String Rotation Check
//!
//! Checks if one string is a valid rotation of another
//! (e.g., "waterbottle" is a rotation of "erbottlewat").

fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() || s1.is_empty() {
        return false;
    }
    let doubled = format!("{}{}", s2, s2);
    doubled.contains(s1)
}

fn main() {
    assert!(is_rotation("erbottlewat", "waterbottle"));
    assert!(!is_rotation("hello", "world"));
    println!("Is rotation: {}", is_rotation("erbottlewat", "waterbottle"));
}
