// https://leetcode.com/problems/increasing-triplet-subsequence/discuss/79004/Concise-Java-solution-with-comments.
// min < mid < nums[i]

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // start with two largest values, as soon as we find a number bigger than both, while both have been updated, return true.
        let mut min = i32::MAX;
        let mut mid = i32::MAX;
        for num in nums {
            if num <= min { min = num; } // update small if n is smaller than both
            else if num <= mid { mid = num; } // update big only if greater than small but smaller than big
            else { return true; } // return if you find a number bigger than both
        }
        return false;
    }
}