// https://leetcode.com/problems/burst-balloons/discuss/76228/Share-some-analysis-and-explanations
//
// dp[left][right]: coins between index left and right (not including left or right)
// dp[left][right] = max(nums[left] * nums[i] * nums[right] + dp[left][i] + dp[i][right]) for i in (left+1..right)

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut ballons = Vec::with_capacity(nums.len() + 1);
        ballons.push(1);
        nums.into_iter()
            .filter(|&x| x != 0)
            .for_each(|x| ballons.push(x));
        ballons.push(1);

        let n = ballons.len();
        let mut dp = vec![vec![0; n]; n];
        for k in 2..n{
            for left in 0..n-k{
                let right = left + k;
                for i in left + 1..right {
                    dp[left][right] = dp[left][right].max(
                        ballons[left] * ballons[i] * ballons[right]
                            + dp[left][i]
                            + dp[i][right]
                    );
                }        
            }
        }
        dp[0][n-1]
    }
}