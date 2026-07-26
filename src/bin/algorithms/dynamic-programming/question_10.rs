//! Rod Cutting
//!
//! Determines the maximum value obtained by cutting a rod of length N,
//! given prices for each piece size, using DP.

fn rod_cutting(prices: &[i32], n: usize) -> i32 {
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        let mut max_val = i32::MIN;
        for j in 0..i {
            max_val = max_val.max(prices[j] + dp[i - j - 1]);
        }
        dp[i] = max_val;
    }
    dp[n]
}

fn main() {
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    assert_eq!(rod_cutting(&prices, 4), 10);
    assert_eq!(rod_cutting(&prices, 8), 22);
    println!("Max rod value (length 8): {}", rod_cutting(&prices, 8));
}
