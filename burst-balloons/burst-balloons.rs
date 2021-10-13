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
        let mut dp = vec![vec![None; n]; n];
        max_coins(&mut dp, &ballons, 0, ballons.len() - 1)
    }
}

fn max_coins(dp: &mut Vec<Vec<Option<i32>>>, nums: &Vec<i32>, left: usize, right: usize) -> i32 {
    match dp[left][right] {
        Some(coins) => coins,
        None => {
            let mut coins = 0;

            for i in (left + 1..right) {
                coins = coins.max(
                    nums[left] * nums[i] * nums[right]
                        + max_coins(dp, nums, left, i)
                        + max_coins(dp, nums, i, right),
                );
            }
            dp[left][right] = Some(coins);
            coins
        }
    }
}
