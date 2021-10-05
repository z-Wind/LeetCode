impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in (0..nums.len()){
            let j = nums[i].abs() as usize - 1;
            if nums[j] < 0{
                return j as i32 + 1;
            }
            nums[j] = -nums[j] 
        }
        0
    }
}