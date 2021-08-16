impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return nums[0];
        }
        
        let mut max_sum = i32::MIN;
        for i in (0..nums.len()){
            let mut sum = nums[i];
            if sum>max_sum{
                max_sum = sum;
            }
            for j in (i+1..nums.len()){
                sum+=nums[j];
                if sum>max_sum{
                    max_sum = sum;
                }
            }
        }
        
        max_sum
    }
}