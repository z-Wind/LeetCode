// https://leetcode.com/problems/increasing-triplet-subsequence/discuss/79004/Concise-Java-solution-with-comments.
// min_2nd < nums[i]
// if min_2nd is updated, it means at least one is smaller than min_2nd.

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // start with two largest values, as soon as we find a number bigger than both, while both have been updated, return true.
        let mut min = i32::MAX;
        let mut min_2nd = i32::MAX;
        for num in nums {
            if num <= min {
                min = num;
            }
            // update min if num is smaller than both
            else if num <= min_2nd {
                min_2nd = num;
            }
            // update min_2nd only if greater than min but smaller than min_2nd
            else {
                // return if you find a number bigger than min_2nd
                return true;
            }
        }
        return false;
    }
}
