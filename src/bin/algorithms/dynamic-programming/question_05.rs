//! Coin Change
//!
//! Finds the minimum number of coins needed to make a target amount using DP.

fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount as usize {
        for &coin in coins {
            if coin as usize <= i {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }
    if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    }
}

fn main() {
    assert_eq!(coin_change(&[1, 2, 5], 11), 3);
    assert_eq!(coin_change(&[2], 3), -1);
    println!("Min coins for 11: {}", coin_change(&[1, 2, 5], 11));
}
