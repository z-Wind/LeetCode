// https://leetcode.com/problems/longest-increasing-subsequence/discuss/74824/JavaPython-Binary-search-O(nlogn)-time-with-explanation/206357
// https://www.cs.princeton.edu/courses/archive/spring13/cos423/lectures/LongestIncreasingSubsequence.pdf

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut piles = Vec::with_capacity(nums.len());
        for num in nums {
            let idx = piles.binary_search(&num).unwrap_or_else(|e| e);
            if idx == piles.len(){
                piles.push(num);
            } else {
                piles[idx] = num;
            }
        }
        piles.len() as i32
    }
}
