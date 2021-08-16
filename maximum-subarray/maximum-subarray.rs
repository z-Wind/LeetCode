// Kadane’s Algorithm
// https://medium.com/@rsinghal757/kadanes-algorithm-dynamic-programming-how-and-why-does-it-work-3fd8849ed73d
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut ans = nums[0];
        let mut last = ans;
        for &i in &nums[1..] {
            last = max(i, last + i);
            ans = max(ans, last);
        }
        ans
    }
}
