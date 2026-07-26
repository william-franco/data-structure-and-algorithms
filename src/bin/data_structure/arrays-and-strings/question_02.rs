//! Remove Duplicates Preserving Order
//!
//! Removes duplicate elements from a `Vec<i32>` while preserving original order,
//! without using `HashSet`.

fn remove_duplicates(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() {
        let val = arr[i];
        let mut j = i + 1;
        while j < arr.len() {
            if arr[j] == val {
                arr.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 2, 4, 1, 5];
    remove_duplicates(&mut data);
    assert_eq!(data, vec![1, 2, 3, 4, 5]);
    println!("Without duplicates: {:?}", data);
}
