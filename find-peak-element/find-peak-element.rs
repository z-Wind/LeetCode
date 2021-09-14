impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 || nums[0] > nums[1]{
            return 0;
        } else if nums[len-1] > nums[len-2]{
            return len as i32 - 1;
        }
        
        let mut start = 0;
        let mut end = len-1;
        while end > start{
            let mid = (start+end)/2;
            if nums[mid+1] > nums[mid]{
                start = mid+1;
            } else {
                end = mid;
            }
        }
        start as i32
    }
}