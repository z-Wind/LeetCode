// https://leetcode.com/problems/missing-number/discuss/69791/4-Line-Simple-Java-Bit-Manipulate-Solution-with-Explaination
// a^b^b = a
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        for i in (0..nums.len()){
            xor = xor ^ (i as i32 + 1) ^ nums[i];
        }
        xor
    }
}