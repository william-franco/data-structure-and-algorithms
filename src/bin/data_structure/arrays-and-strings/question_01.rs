//! Second Largest Element
//!
//! Finds the second largest element in a `Vec<i32>` without sorting the vector.

fn second_largest(arr: &[i32]) -> Option<i32> {
    if arr.len() < 2 {
        return None;
    }
    let mut first = i32::MIN;
    let mut second = i32::MIN;

    for &val in arr {
        if val > first {
            second = first;
            first = val;
        } else if val > second && val != first {
            second = val;
        }
    }
    if second == i32::MIN {
        None
    } else {
        Some(second)
    }
}

fn main() {
    assert_eq!(second_largest(&[12, 35, 1, 10, 34, 1]), Some(34));
    assert_eq!(second_largest(&[10, 10, 10]), None);
    println!("Second largest: {:?}", second_largest(&[12, 35, 1, 10, 34, 1]));
}
