//! Bubble Sort
//!
//! Sorts a `Vec<i32>` in ascending order without using the standard `.sort()` method.

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut data = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut data);
    assert_eq!(data, vec![11, 12, 22, 25, 34, 64, 90]);
    println!("Sorted: {:?}", data);
}
