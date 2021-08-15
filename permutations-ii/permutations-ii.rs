use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1{
            return vec![nums];
        }
        let mut ans:HashSet<Vec<i32>> = HashSet::new();
        push(&mut ans, &mut nums, 0);
        ans.into_iter().collect()
        
    }
}

fn push(ans: &mut HashSet<Vec<i32>>, nums:&mut Vec<i32>, pos:usize){
    if pos == nums.len(){
        ans.insert(nums.clone());
    } else{
        for i in (pos..nums.len()){
            nums.swap(pos,i);
            push(ans, nums, pos+1);
            nums.swap(pos,i);
        }
    }
}