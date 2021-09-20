impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left:usize = 0;
        let mut right:usize = 0;
        let mut sum = 0;
        let mut min_len = i32::MAX;
        while right < nums.len(){
            sum += nums[right];
            right += 1;
            while sum >= target{
                min_len = min_len.min((right-left) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if min_len == i32::MAX{
            return 0;
        }
        min_len
    }
}