use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let complement = target - x;
            if let Some(&j) = hashmap.get(&complement){
                return vec![i as i32,j as i32];
            }
            hashmap.insert(x,i);
        }
        return vec![];
    }
}