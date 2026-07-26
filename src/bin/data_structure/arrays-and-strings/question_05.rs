//! Array Rotation
//!
//! Rotates the elements of a `Vec<i32>` k positions to the right.

fn rotate_right(arr: &mut [i32], k: usize) {
    let n = arr.len();
    if n == 0 {
        return;
    }
    let k = k % n;
    reverse(arr, 0, n - 1);
    reverse(arr, 0, k - 1);
    reverse(arr, k, n - 1);
}

fn reverse(arr: &mut [i32], start: usize, end: usize) {
    let mut i = start;
    let mut j = end;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_right(&mut data, 3);
    assert_eq!(data, vec![5, 6, 7, 1, 2, 3, 4]);
    println!("Rotated: {:?}", data);
}
