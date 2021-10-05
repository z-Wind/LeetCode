impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lastNonZeroFoundAt = 0;
        for cur in (0..nums.len()) {
            if (nums[cur] != 0) {
                nums.swap(lastNonZeroFoundAt, cur);
                lastNonZeroFoundAt += 1;
            }
        }
    }
}