// https://leetcode.com/problems/total-hamming-distance/discuss/96226/Java-O(n)-time-O(1)-Space

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        (0..32).fold(0, |total, i| {
            let bitCount = nums.iter().fold(0, |acc, num| acc + ((num >> i) & 1));
            total + bitCount * (n - bitCount)
        })
    }
}
