// dp[i] = nums[i] + max(dp[i+2] + dp[i+3])

use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3{
            return *nums.iter().max().unwrap();
        }
        let mut dp:Vec<i32> = vec![0;n];
        
        dp[n-1] = nums[n-1];
        dp[n-2] = nums[n-2];
        dp[n-3] = nums[n-3] + dp[n-1];
        for i in (0..n-3).rev(){
            dp[i] = nums[i] + max(dp[i+2], dp[i+3]);
        }
        
        max(dp[0], dp[1])
    }
}