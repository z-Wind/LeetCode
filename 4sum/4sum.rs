use std::collections::HashSet;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        k_sum(&nums[..], target, 4)
    }
}

fn k_sum(mut nums: &[i32], target: i32, k:usize) -> Vec<Vec<i32>> {
    if nums.len() < k || (nums[0] * k as i32) > target || (nums.last().unwrap() * k as i32) < target {
        return vec![];
    }
    if k == 2{
        return two_sum(&nums[..], target)
    }
    
    let mut ans:Vec<Vec<i32>> = vec![];

    for i in (0..nums.len()-k+1){
        if i != 0 && nums[i-1] == nums[i]{
            continue;
        }
        for mut v in k_sum(&nums[i+1..], target-nums[i], k-1){
            v.extend_from_slice(&nums[i..i+1]);
            ans.push(v);
        }
    }

    ans
}

fn two_sum(mut nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut ans:Vec<Vec<i32>> = vec![];

    let mut left=0;
    let mut right=nums.len()-1;

    while left < right{
        let sum = nums[left] + nums[right];

        if (sum < target) {
            left+=1;
            continue;
        } else if (sum > target) {
            right-=1;
            continue;
        }

        ans.push(vec![nums[left], nums[right]]);
        left+=1;
        right-=1;

        while left < right && nums[left-1] == nums[left]{
            left+=1;
        }
        while left < right && nums[right+1] == nums[right]{
            right-=1;
        }
    }

    ans
}