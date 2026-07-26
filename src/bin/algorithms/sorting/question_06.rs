//! Heap Sort
//!
//! Sorts a `Vec<i32>` by building a max heap manually (without `BinaryHeap`)
//! and extracting elements in sorted order.

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != root {
        arr.swap(root, largest);
        heapify(arr, heap_size, largest);
    }
}

fn main() {
    let mut data = vec![12, 11, 13, 5, 6, 7];
    heap_sort(&mut data);
    assert_eq!(data, vec![5, 6, 7, 11, 12, 13]);
    println!("Sorted: {:?}", data);
}
