//! Longest Increasing Subsequence (LIS)
//!
//! Finds the length of the longest strictly increasing subsequence using DP.

fn lis(arr: &[i32]) -> usize {
    if arr.is_empty() {
        return 0;
    }
    let mut dp = vec![1; arr.len()];
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[j] < arr[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    assert_eq!(lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(lis(&[0, 1, 0, 3, 2, 3]), 4);
    println!("LIS length: {}", lis(&[10, 9, 2, 5, 3, 7, 101, 18]));
}
