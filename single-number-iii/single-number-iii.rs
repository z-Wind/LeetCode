// https://leetcode.com/problems/single-number-iii/discuss/68900/Accepted-C%2B%2BJava-O(n)-time-O(1)-space-Easy-Solution-with-Detail-Explanations

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0,|acc, x| acc^x);
        let group_condition = xor & -xor;
        
        let mut result = vec![0,0];
        for num in nums{
            if num & group_condition == 0{
                result[0] ^= num;
            } else {
                result[1] ^= num;
            }
        }
        
        result
    }
}