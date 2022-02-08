// https://leetcode.com/problems/target-sum/discuss/455024/DP-IS-EASY!-5-Steps-to-Think-Through-DP-Questions.

use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();
        find_target_sum_ways(&mut memo, &nums, target, nums.len() - 1, 0)
    }
}

fn find_target_sum_ways(
    memo: &mut HashMap<(usize, i32), i32>,
    nums: &[i32],
    target: i32,
    index: usize,
    curr_sum: i32,
) -> i32 {
    let key = (index, curr_sum);
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }

    if index >= nums.len() {
        if curr_sum == target {
            return 1;
        } else {
            return 0;
        }
    }

    let positive = find_target_sum_ways(memo, nums, target, index - 1, curr_sum + nums[index]);
    let negative = find_target_sum_ways(memo, nums, target, index - 1, curr_sum - nums[index]);

    let res = positive + negative;
    memo.insert(key, res);
    return res;
}
