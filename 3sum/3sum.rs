use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = vec![];
        
        if nums.len() < 3 {
            return ans;
        }
        nums.sort();
        
        let mut i=0;
        for i in (0..nums.len()-2){
            if i>0 && nums[i-1] == nums[i]{
                continue;
            }
            
            let mut j=i+1;
            let mut k=nums.len()-1;
            
            while j < k{
                let sum = nums[i] + nums[j] + nums[k];
                
                if (sum < 0) {
                    j+=1;
                    continue;
                } else if (sum > 0) {
                    k-=1;
                    continue;
                }
                
                ans.push(vec![nums[i], nums[j], nums[k]]);
                j+=1;
                k-=1;
                
                while j < k && nums[j-1] == nums[j]{
                    j+=1;
                }
                while j < k && nums[k+1] == nums[k]{
                    k-=1;
                }
            }
        }
        
        ans
    }
}

