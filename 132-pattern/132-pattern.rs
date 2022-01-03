// https://leetcode.com/problems/132-pattern/discuss/94071/Single-pass-C%2B%2B-O(n)-space-and-time-solution-(8-lines)-with-detailed-explanation.

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut s3 = i32::MIN;
        let mut s2 = Vec::new();
        for &num in nums.iter().rev(){
            if num < s3 {
                return true;
            }
            while !s2.is_empty() && num > *s2.last().unwrap() { 
              s3 = s2.pop().unwrap(); 
            }
            s2.push(num);
        }
        return false;
    }
}