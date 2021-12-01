use std::collections::HashMap;
use rand::seq::SliceRandom;

struct Solution {
    map:HashMap<i32,Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for i in 0..nums.len(){
            map.entry(nums[i]).or_insert(Vec::new()).push(i as i32);
        }
        Self{
            map,
        }
    }
    
    fn pick(&self, target: i32) -> i32 {
        *self
            .map
            .get(&target)
            .unwrap()
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */