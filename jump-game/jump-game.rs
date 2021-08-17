use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_index = 0;
        let mut last = 0;
        let mut step = 0;
        for i in (0..nums.len()-1){
            max_index = max(max_index, i+nums[i] as usize);            
            if i == last{
                step += 1;
                last = max_index;
                if last >= nums.len()-1{
                    break;
                } else if last == i{
                    break;
                }
            }
        }
        if last < nums.len()-1{
            return false;
        }
        //println!("step:{}",step);
        true
    }
}