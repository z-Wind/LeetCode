// find min[0..i] < nums[i] < max[i+1..]
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3{
            return false;
        }
        
        let mut max = vec![0;n];
        max[n-1] = nums[n-1];
        for i in (0..n-1).rev(){
            max[i] = nums[i].max(max[i+1]);
        }
        
        let mut min = nums[0];
        for i in 1..n-1{
            if min < nums[i] && nums[i] < max[i]{
                return true;
            }
            min = min.min(nums[i]);
        }
        
        false
    }
}