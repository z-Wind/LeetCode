use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp:HashMap<usize,i32> = HashMap::new();
        max(backtrack(&mut dp, &nums,0), backtrack(&mut dp, &nums,1))
    }
}

fn backtrack(dp:&mut HashMap<usize,i32>, nums: &Vec<i32>, start:usize) -> i32 {
    if start >= nums.len(){
        return 0
    }
    if dp.contains_key(&start){
        return *dp.get(&start).unwrap();
    }
    
    let ans = nums[start] + max(backtrack(dp,nums,start+2), backtrack(dp,nums,start+3));
    dp.insert(start,ans);
    ans
}