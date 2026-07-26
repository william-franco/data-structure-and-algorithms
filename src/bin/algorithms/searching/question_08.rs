//! Interpolation Search
//!
//! Finds a value in a uniformly distributed sorted `Vec<i32>` using interpolation search.

fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len().saturating_sub(1);

    while low <= high && target >= arr[low] && target <= arr[high] {
        if low == high {
            return if arr[low] == target { Some(low) } else { None };
        }
        let pos = low
            + ((target - arr[low]) as f64 / (arr[high] - arr[low]) as f64 * (high - low) as f64)
                as usize;
        let pos = pos.min(high);
        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            low = pos + 1;
        } else {
            high = pos.saturating_sub(1);
        }
    }
    None
}

fn main() {
    let data: Vec<i32> = (0..100).map(|i| i * 10).collect();
    assert_eq!(interpolation_search(&data, 500), Some(50));
    assert_eq!(interpolation_search(&data, 555), None);
    println!("Found: {:?}", interpolation_search(&data, 300));
}
