impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new(); 
        let mut temp:Vec<i32> = Vec::new(); 
        backtrack(&mut ans, &nums, &mut temp, 0);
        ans
    }
}


fn backtrack(ans: &mut Vec<Vec<i32>>, nums:&Vec<i32>, temp:&mut Vec<i32>, pos:usize){
    ans.push(temp.clone());
    if pos == nums.len(){
        return;
    }

    for i in (pos..nums.len()){
        temp.push(nums[i]);
        backtrack(ans, nums, temp, i+1);
        temp.pop();
    }
}