impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1{
            return vec![nums];
        }
        let mut ans:Vec<Vec<i32>> = Vec::new();
        push(&mut ans, vec![], nums);
        ans
        
    }
}

fn push(ans: &mut Vec<Vec<i32>>, mut v:Vec<i32>, mut nums:Vec<i32>){
    if nums.len() == 0{
        ans.push(v);
    } else{
        for _ in (0..nums.len()){
            let mut tmp = v.clone();
            tmp.push(nums[0]);
            let (_, right) = nums.split_at(1);
            push(ans, tmp, right.to_vec());
            nums.rotate_left(1);
        }
    }
}