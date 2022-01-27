// https://leetcode.com/problems/predict-the-winner/discuss/96828/JAVA-9-lines-DP-solution-easy-to-understand-with-improvement-to-O(N)-space-complexity.

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if (n & 1) == 0 { 
            return true; 
        }
        let mut dp = vec![0;n];
        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i] = nums[i];
                } else {
                    dp[j] = (nums[i] - dp[j]).max(nums[j] - dp[j - 1]);
                }
            }
        }
        return dp[n - 1] >= 0;
    }
}