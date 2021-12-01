// Reservoir sampling

use std::collections::HashMap;
use rand::{thread_rng, Rng};

struct Solution {
    nums:Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self{
            nums,
        }
    }
    
    fn pick(&self, target: i32) -> i32 {
        let mut rng = thread_rng();
        let mut idx = 0;
        let mut count = 0;
        for i in 0..self.nums.len() {
            if self.nums[i] != target{
                continue;
            }
            count += 1;
            if rng.gen_range(0, count) == 0{
                idx = i;
            }
        }
        
        idx as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */