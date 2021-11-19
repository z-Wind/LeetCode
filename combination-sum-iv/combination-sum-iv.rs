// https://leetcode.com/problems/combination-sum-iv/discuss/85036/1ms-Java-DP-Solution-with-Detailed-Explanation
// comb[target] = sum(comb[target - nums[i]]), where 0 <= i < nums.length, and target >= nums[i]
// comb[4] = comb[4-1] + comb[4-2] + comb[4-3] = comb[3] + comb[2] + comb[1]

use std::collections::HashMap;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp:HashMap<i32, i32> = HashMap::with_capacity(target as usize);
        combination_sum4(&mut dp, &nums, target)
    }
}

fn combination_sum4(dp:&mut HashMap<i32, i32>, nums: &Vec<i32>, target: i32) -> i32{
    if target == 0 {
        return 1;
    }
    if dp.contains_key(&target){
        return *dp.get(&target).unwrap();
    }
    let mut res = 0;
    for i in 0..nums.len() {
        if target >= nums[i] {
            let new_target = target - nums[i];
            let comb = combination_sum4(dp, nums, new_target);
            dp.insert(new_target, comb);
            res += comb;
        }
    }
    return res;
}