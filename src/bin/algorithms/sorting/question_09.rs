//! Shell Sort
//!
//! Sorts a `Vec<i32>` using Shell Sort with a decreasing gap sequence.

fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j, j - gap);
                j -= gap;
            }
        }
        gap /= 2;
    }
}

fn main() {
    let mut data = vec![9, 8, 3, 7, 5, 6, 4, 1];
    shell_sort(&mut data);
    assert_eq!(data, vec![1, 3, 4, 5, 6, 7, 8, 9]);
    println!("Sorted: {:?}", data);
}
