// https://leetcode.com/problems/partition-equal-subset-sum/discuss/276278/Python-DP-and-(DFS%2BMemo)

use std::collections::HashMap;
impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().fold(0, |acc, x| acc+x);
        // println!("{}", sum);
        // sum % 2 == 1
        if sum & 1 == 1{
            return false;
        }
        
        nums.sort_unstable_by(|a,b| b.cmp(a));
        let mut memo = HashMap::new();
        memo.insert(0, true);
        dfs(&mut memo, &nums, 0, sum>>1)
    }
}

fn dfs(memo:&mut HashMap<i32,bool>, nums:&Vec<i32>, start:usize, x:i32) -> bool{
    match memo.get(&x){
        None => {
            memo.insert(x, false);
            if x > 0{
                for i in start..nums.len(){
                    if dfs(memo, nums, i+1, x-nums[i]){
                        memo.insert(x, true);
                        break;
                    }
                }
            }
            *memo.get(&x).unwrap()
        },
        Some(can) => *can,
    }
}
