impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in (0..nums.len()).rev(){
            if nums[i] == val{
                nums.remove(i);
            }
        }
        nums.len() as i32
    }
}