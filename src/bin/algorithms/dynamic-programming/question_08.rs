//! Equal Subset Sum Partition
//!
//! Checks if a `Vec<i32>` can be partitioned into two subsets with equal sums using DP.

fn can_partition(arr: &[i32]) -> bool {
    let total: i32 = arr.iter().sum();
    if total % 2 != 0 {
        return false;
    }
    let target = total / 2;
    let mut dp = vec![false; (target + 1) as usize];
    dp[0] = true;

    for &num in arr {
        for j in (num..=target).rev() {
            dp[j as usize] = dp[j as usize] || dp[(j - num) as usize];
        }
    }
    dp[target as usize]
}

fn main() {
    assert!(can_partition(&[1, 5, 11, 5]));
    assert!(!can_partition(&[1, 2, 3, 5]));
    println!("Can partition [1,5,11,5]: {}", can_partition(&[1, 5, 11, 5]));
}
