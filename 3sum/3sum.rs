use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        let mut hashmap = HashMap::new();
        
        for i in (0..nums.len()){
            for j in (i+1..nums.len()){
                let complement = - (nums[i] + nums[j]);
                if let Some(&k) = hashmap.get(&complement){
                    if i != j && i != k && j != k{
                        let mut v = vec![nums[i],nums[j],nums[k]];
                        v.sort();
                        ans.insert(v);
                    }
                }
                hashmap.insert(nums[j],j);
            }
        }
        
        ans.into_iter().collect()
    }
}

