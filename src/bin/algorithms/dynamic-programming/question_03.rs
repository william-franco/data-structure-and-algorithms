//! Longest Common Subsequence (LCS)
//!
//! Finds the length of the longest common subsequence between two strings using DP.

fn lcs(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

fn main() {
    assert_eq!(lcs("ABCDGH", "AEDFHR"), 3);
    assert_eq!(lcs("AGGTAB", "GXTXAYB"), 4);
    println!("LCS length: {}", lcs("ABCDGH", "AEDFHR"));
}
