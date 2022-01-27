// https://leetcode.com/problems/predict-the-winner/discuss/96828/JAVA-9-lines-DP-solution-easy-to-understand-with-improvement-to-O(N)-space-complexity.

// dp[i][j] saves how much more scores that the first-in-action player will get from i to j than the second player
// dp[i][i] = nums[i]
// dp[i][j] = max(nums[i] - dp[i + 1][j], nums[j] - dp[i][j - 1]);
//
// dp[i][j-1]  dp[i][j]
//             dp[i+1][j]
//
// dp[j-1]     dp[j] (update)
//             dp[j]

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        
        // [1,2,3,4,5,6,7,8]
        //  o x o x o x o x
        // player 1 could always select the even elements or odd elements
        // if even elements larger or equal than odd elements
        // select even elements or select odd elements
        // so player 1 always wins if the lengh of nums is even
        //            n % 2 == 0
        if n == 1 || (n & 1) == 0 { 
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