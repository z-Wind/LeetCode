// https://leetcode.com/problems/combination-sum-iv/discuss/85036/1ms-Java-DP-Solution-with-Detailed-Explanation
// comb[target] = sum(comb[target - nums[i]]), where 0 <= i < nums.length, and target >= nums[i]
// comb[4] = comb[4-1] + comb[4-2] + comb[4-3] = comb[3] + comb[2] + comb[1]

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![-1;target as usize + 1];
        combination_sum4(&mut dp, &nums, target)
    }
}

fn combination_sum4(dp:&mut Vec<i32>, nums: &Vec<i32>, target: i32) -> i32{
    if target == 0 {
        return 1;
    }
    if dp[target as usize] != -1{
        return dp[target as usize];
    }
    let mut res = 0;
    for i in 0..nums.len() {
        if target >= nums[i] {
            let idx = (target - nums[i]) as usize;
            dp[idx] = combination_sum4(dp, nums, target - nums[i]);
            res += dp[idx];
        }
    }
    return res;
}