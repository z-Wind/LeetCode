impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut temp:Vec<i32> = Vec::new();
        nums.sort_unstable();
        //println!("{:?}",nums);
        backtrack(&mut ans, &nums, &mut temp, 0);
        ans
    }
}

fn backtrack(ans:&mut Vec<Vec<i32>>, nums:&Vec<i32>, temp:&mut Vec<i32>, pos:usize){
    //println!("pos:{}, temp:{:?}",pos,temp);
    ans.push(temp.clone());
    if pos == nums.len(){
        return;
    }
    for i in (pos..nums.len()){
        if i==pos || nums[i] != nums[i-1]{
            temp.push(nums[i]);
            backtrack(ans, nums, temp, i+1);
            temp.pop();
        }
    }
}