// https://github.com/z-Wind/LeetCode/blob/main/496-next-greater-element-i/496-next-greater-element-i.rs

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut stack = Vec::new();
        for i in (0..n).rev().cycle().take(n * 2) {
            while !stack.is_empty() && *stack.last().unwrap() <= nums[i] {
                stack.pop();
            }
            if let Some(&x) = stack.last() {
                ans[i] = x;
            }
            stack.push(nums[i]);
        }

        ans
    }
}
