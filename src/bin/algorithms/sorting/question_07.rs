//! Counting Sort
//!
//! Sorts a `Vec<i32>` of non-negative integers within a known range
//! using the counting sort algorithm.

fn counting_sort(arr: &mut [i32], max_val: i32) {
    let mut count = vec![0usize; (max_val + 1) as usize];
    for &val in arr.iter() {
        count[val as usize] += 1;
    }
    let mut idx = 0;
    for (val, &freq) in count.iter().enumerate() {
        for _ in 0..freq {
            arr[idx] = val as i32;
            idx += 1;
        }
    }
}

fn main() {
    let mut data = vec![4, 2, 2, 8, 3, 3, 1];
    counting_sort(&mut data, 8);
    assert_eq!(data, vec![1, 2, 2, 3, 3, 4, 8]);
    println!("Sorted: {:?}", data);
}
