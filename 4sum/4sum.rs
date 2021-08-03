use std::collections::HashSet;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        nums.sort();
        
        let mut ans = HashSet::new();
        
        for i in (0..nums.len()-3){
            let mut temp = three_sum(&nums[i+1..], target-nums[i]);
            for mut v in temp.iter_mut(){
                v.push(nums[i]);
                ans.insert(v.to_vec());
            }
        }
        
        ans.into_iter().collect()
    }
}

fn three_sum(mut nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut ans:Vec<Vec<i32>> = vec![];

    let mut i=0;
    for i in (0..nums.len()-2){
        if i>0 && nums[i-1] == nums[i]{
            continue;
        }

        let mut j=i+1;
        let mut k=nums.len()-1;

        while j < k{
            let sum = nums[i] + nums[j] + nums[k];

            if (sum < target) {
                j+=1;
                continue;
            } else if (sum > target) {
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