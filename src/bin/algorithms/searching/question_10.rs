//! Quickselect — K-th Smallest Element
//!
//! Finds the k-th smallest element in an unsorted `Vec<i32>` without sorting the entire vector.

fn quickselect(arr: &mut [i32], k: usize) -> i32 {
    let target = k - 1;
    let mut left = 0;
    let mut right = arr.len() - 1;

    loop {
        let pivot_index = partition(arr, left, right);
        if pivot_index == target {
            return arr[pivot_index];
        } else if pivot_index < target {
            left = pivot_index + 1;
        } else {
            right = pivot_index - 1;
        }
    }
}

fn partition(arr: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = arr[right];
    let mut i = left;
    for j in left..right {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    i
}

fn main() {
    let mut data = vec![7, 10, 4, 3, 20, 15];
    assert_eq!(quickselect(&mut data, 3), 7);
    let mut data2 = vec![3, 2, 1, 5, 6, 4];
    assert_eq!(quickselect(&mut data2, 2), 2);
    println!("3rd smallest: {}", quickselect(&mut vec![7, 10, 4, 3, 20, 15], 3));
}
