impl Solution {
    pub fn find_subsequences(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut temp = Vec::new();
        find_subsequences(&mut ans, &mut temp, &nums, 0);

        ans
    }
}

fn find_subsequences(ans: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, i: usize) {
    if i == nums.len() {
        if temp.len() > 1 {
            ans.push(temp.clone());
        }
        return;
    }
    
    if temp.is_empty() || nums[i] >= temp.last().cloned().unwrap(){
        temp.push(nums[i]);
        find_subsequences(ans, temp, nums, i + 1);
        temp.pop();
    } 
    
    if temp.len() > 0 && nums[i] == temp.last().cloned().unwrap() {
        return;
    }
    
    find_subsequences(ans, temp, nums, i + 1);
}