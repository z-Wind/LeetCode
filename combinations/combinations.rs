use std::cmp::min;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut nums:Vec<i32> = (1..=n).collect();
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut temp:Vec<i32> = Vec::new(); 
        backtrack(&mut ans, &nums, &mut temp, 0, k as usize);
        ans
    }
}


fn backtrack(ans: &mut Vec<Vec<i32>>, nums:&Vec<i32>, temp:&mut Vec<i32>, pos:usize, len:usize){
    if temp.len() == len{
        ans.push(temp.clone());
        return;
    }

    for i in (pos..nums.len()){
        temp.push(nums[i]);
        backtrack(ans, nums, temp, i+1, len);
        temp.pop();
    }
}