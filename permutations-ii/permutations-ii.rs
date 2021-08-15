use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1{
            return vec![nums];
        }
        let mut ans:Vec<Vec<i32>> = Vec::new();
        push(&mut ans, &mut nums, 0);
        ans
        
    }
}

fn push(ans: &mut Vec<Vec<i32>>, nums:&mut Vec<i32>, pos:usize){
    if pos == nums.len(){
        //println!("==============");
        ans.push(nums.clone());
    } else{
        let mut set = HashSet::new();
        for i in (pos..nums.len()){
            if set.contains(&nums[i]){
                continue;
            }
            set.insert(nums[i]);
            
            nums.swap(pos,i);
            //println!("{:?} set:{:?} pos:{} i:{}", &nums[..pos+1], set, pos, i);
            push(ans, nums, pos+1);
            nums.swap(pos,i);
        }
    }
}