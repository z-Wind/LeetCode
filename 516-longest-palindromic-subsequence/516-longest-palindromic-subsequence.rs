// https://leetcode.com/problems/longest-palindromic-subsequence/discuss/99101/Straight-forward-Java-DP-solution

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                dp[i][j] = if s[i] == s[j] {
                    dp[i + 1][j - 1] + 2
                } else {
                    dp[i + 1][j].max(dp[i][j - 1])
                };
            }
        }

        dp[0][n - 1]
    }
}


