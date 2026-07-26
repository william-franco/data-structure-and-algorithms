//! Minimum Path Sum in Grid
//!
//! Finds the minimum sum path from top-left to bottom-right of a grid,
//! moving only down or right, using DP.

fn min_path_sum(grid: &[Vec<i32>]) -> i32 {
    if grid.is_empty() {
        return 0;
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0; cols]; rows];

    dp[0][0] = grid[0][0];
    for j in 1..cols {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }
    for i in 1..rows {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }
    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
        }
    }
    dp[rows - 1][cols - 1]
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(min_path_sum(&grid), 7);
    println!("Min path sum: {}", min_path_sum(&grid));
}
