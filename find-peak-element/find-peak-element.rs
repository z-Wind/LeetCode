impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 || nums[0] > nums[1]{
            return 0;
        } else if nums[len-1] > nums[len-2]{
            return len as i32 - 1;
        }
        
        for (i,w) in nums.windows(3).enumerate(){
            if w[0] < w[1] && w[1] > w[2]{
                return i as i32 + 1;
            }
        }
        -1
    }
}