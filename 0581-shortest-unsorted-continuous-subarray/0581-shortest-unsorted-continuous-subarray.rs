// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/solutions/103057/java-o-n-time-o-1-space/

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let ((left, _), (right, _)) = (0..n).fold(
            ((n + 1, i32::MAX), (n, i32::MIN)),
            |((mut left, mut min), (mut right, mut max)), i| {
                if nums[n - 1 - i] <= min {
                    min = nums[n - 1 - i];
                } else {
                    left = n - 1 - i;
                }
                if nums[i] >= max {
                    max = nums[i];
                } else {
                    right = i;
                }

                ((left, min), (right, max))
            },
        );

        // println!("{},{}", left, right);
        (right - left + 1) as i32
    }
}
