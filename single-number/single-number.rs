impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut val:i32 = 0;
        for num in nums{
            val ^= num;
        }
        val
    }
}