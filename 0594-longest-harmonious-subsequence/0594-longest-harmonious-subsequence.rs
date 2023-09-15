// https://leetcode.com/problems/longest-harmonious-subsequence/solutions/103499/three-c-solution-run-time-with-explanation/

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        // println!("{:?}", nums);

        let mut result = 0;
        let mut min_i = 0;
        let mut max_i = 0;
        for (i, &num) in nums.iter().enumerate().skip(1) {
            // println!(
            //     "cur:{}, min:{} ,max:{}, result:{}",
            //     num, nums[min_i], nums[max_i], result
            // );
            if num != nums[min_i] + 1 {
                min_i = max_i;
            }
            if num == nums[min_i] + 1 {
                result = result.max((i - min_i + 1) as i32);
            }
            if num != nums[i - 1] {
                max_i = i;
            }
        }

        result
    }
}
