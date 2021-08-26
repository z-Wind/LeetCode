impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut temp:Vec<i32> = Vec::new();
        nums.sort_unstable();
        //println!("{:?}",nums);
        backtrack(&mut ans, &nums[..], &mut temp);
        ans
    }
}

fn backtrack(ans:&mut Vec<Vec<i32>>, nums:&[i32], temp:&mut Vec<i32>){
    ans.push(temp.clone());
    if nums.is_empty(){
        return;
    }
    for i in (0..nums.len()){
        if i==0 || nums[i] != nums[i-1]{
            temp.push(nums[i]);
            backtrack(ans, &nums[i+1..], temp);
            temp.pop();
        }
    }
}