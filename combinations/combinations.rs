use std::cmp::min;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut nums:Vec<i32> = Vec::new();
        push(&mut ans, &mut nums, 0, k as usize, n as usize);
        ans
    }
}

fn push(ans: &mut Vec<Vec<i32>>, nums:&mut Vec<i32>, start:usize, len:usize, numlen:usize){
    //println!("{:?} {}==?{}", nums, nums.len(), len);
    if nums.len() == len{
        ans.push(nums.clone());
    } else{
        for i in (start..numlen){
            nums.push(i as i32 +1);
            push(ans, nums, i+1, len, numlen);
            nums.pop();
        }
    }
}