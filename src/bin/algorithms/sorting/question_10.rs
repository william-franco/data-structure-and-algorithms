//! Sorting Algorithm Comparison
//!
//! Compares execution time of at least three sorting algorithms
//! on vectors of increasing sizes using `std::time::Instant`.

use std::time::Instant;

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

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

fn generate_data(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}

fn benchmark<F>(mut data: Vec<i32>, sort_fn: F, name: &str) -> std::time::Duration
where
    F: FnOnce(&mut [i32]),
{
    let start = Instant::now();
    sort_fn(&mut data);
    let elapsed = start.elapsed();
    println!("  {}: {:?}", name, elapsed);
    elapsed
}

fn main() {
    let sizes = [100, 500, 1000];

    for &size in &sizes {
        println!("\nVector size: {}", size);

        let data = generate_data(size);
        benchmark(data, bubble_sort, "Bubble Sort");

        let data = generate_data(size);
        benchmark(data, insertion_sort, "Insertion Sort");

        let data = generate_data(size);
        benchmark(data, quick_sort, "Quick Sort");
    }
}
