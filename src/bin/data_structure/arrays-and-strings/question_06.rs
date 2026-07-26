//! Majority Element (Boyer-Moore Voting)
//!
//! Finds the element appearing more than N/2 times using the Boyer-Moore voting algorithm.

fn majority_element(arr: &[i32]) -> i32 {
    let mut candidate = arr[0];
    let mut count = 1;

    for &val in &arr[1..] {
        if count == 0 {
            candidate = val;
            count = 1;
        } else if val == candidate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    candidate
}

fn main() {
    assert_eq!(majority_element(&[3, 3, 4, 2, 4, 4, 2, 4, 4]), 4);
    assert_eq!(majority_element(&[2, 2, 1, 1, 1, 2, 2]), 2);
    println!("Majority: {}", majority_element(&[3, 3, 4, 2, 4, 4, 2, 4, 4]));
}
