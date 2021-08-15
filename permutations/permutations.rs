impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1{
            return vec![nums];
        }
        let mut ans:Vec<Vec<i32>> = Vec::new();
        push(&mut ans, nums, 0);
        ans
        
    }
}

fn push(ans: &mut Vec<Vec<i32>>, nums:Vec<i32>, pos:usize){
    if pos == nums.len(){
        ans.push(nums);
    } else{
        for i in (pos..nums.len()){
            let mut nums = nums.clone();
            nums.swap(pos,i);
            push(ans, nums, pos+1);
        }
    }
}