// dp[prev][start][pos] = if nums[prev] < nums[start] 1+dp[start][next][neg] else 0
// dp[prev][start][neg] = if nums[prev] > nums[start] 1+dp[start][next][pos] else 0

use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1{
            return 1;
        }
        let mut dp:HashMap<(usize,usize,bool),i32> = HashMap::new();
        let mut ans = 0;
        for i in 1..nums.len(){
            ans = ans.max(1+max(
                wiggle_max_length(&mut dp, &nums, i, i-1, true),
                wiggle_max_length(&mut dp, &nums, i, i-1, false),
            ))
        }
        ans
    }
}

fn wiggle_max_length(dp:&mut HashMap<(usize,usize,bool),i32>, nums: &Vec<i32>, start: usize, prev: usize, is_larger: bool) -> i32 {
    if start == nums.len() {
        return 0;
    }
    if dp.contains_key(&(prev,start,is_larger)){
        return *dp.get(&(prev,start,is_larger)).unwrap();
    }
    
    // println!("{:2} vs {:2}: {}", nums[prev], nums[start], is_larger);

    let mut ans = 0;
    if is_larger && nums[start] > nums[prev] {
        ans = 1 + wiggle_max_length(dp, nums, start + 1, start, !is_larger);
    } else if !is_larger && nums[start] < nums[prev] {
        ans = 1 + wiggle_max_length(dp, nums, start + 1, start, !is_larger);
    }
    ans = ans.max(wiggle_max_length(dp, nums, start + 1, prev, is_larger));
    dp.insert((prev,start,is_larger), ans);
    // println!("{}:{:?}", nums[start], dp);
    ans
}
