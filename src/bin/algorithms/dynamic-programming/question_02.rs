//! 0/1 Knapsack
//!
//! Solves the 0/1 Knapsack problem using dynamic programming to maximize total value
//! without exceeding capacity.

fn knapsack(weights: &[i32], values: &[i32], capacity: i32) -> i32 {
    let n = weights.len();
    let mut dp = vec![vec![0; (capacity + 1) as usize]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            if weights[i - 1] <= w {
                dp[i][w as usize] = dp[i - 1][w as usize]
                    .max(values[i - 1] + dp[i - 1][(w - weights[i - 1]) as usize]);
            } else {
                dp[i][w as usize] = dp[i - 1][w as usize];
            }
        }
    }
    dp[n][capacity as usize]
}

fn main() {
    let weights = vec![2, 3, 4, 5];
    let values = vec![3, 4, 5, 6];
    assert_eq!(knapsack(&weights, &values, 5), 7);
    assert_eq!(knapsack(&weights, &values, 8), 10);
    println!("Max value: {}", knapsack(&weights, &values, 8));
}
