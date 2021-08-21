use std::cmp::min;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut nums:Vec<i32> = (1..=n).collect();
        let mut ans:Vec<Vec<i32>> = Vec::new();
        push(&mut ans, &mut nums, 0, k as usize, 0);
        ans
    }
}

fn push(ans: &mut Vec<Vec<i32>>, nums:&mut Vec<i32>, pos:usize, len:usize, start:usize){
    if pos == len{
        ans.push(nums[..len].to_vec());
        return;
    }
    
    if nums.len() - start < len - pos{
        return;
    }
    
    for i in (start..nums.len()){
        nums.swap(pos,i);
        push(ans, nums, pos+1, len, i+1);
        nums.swap(pos,i);
    }
}