//! Radix Sort
//!
//! Sorts a `Vec<u32>` by processing digits from least to most significant.

fn radix_sort(arr: &mut [u32]) {
    if arr.is_empty() {
        return;
    }
    let max = *arr.iter().max().unwrap();
    let mut exp = 1u32;
    while max / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [u32], exp: u32) {
    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut count = [0usize; 10];

    for &val in arr.iter() {
        let digit = ((val / exp) % 10) as usize;
        count[digit] += 1;
    }
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    for &val in arr.iter().rev() {
        let digit = ((val / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = val;
    }
    arr.copy_from_slice(&output);
}

fn main() {
    let mut data = vec![170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort(&mut data);
    assert_eq!(data, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    println!("Sorted: {:?}", data);
}
