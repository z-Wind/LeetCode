// Process: 1001 | -1001
// Fail: i32::MAX

impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 0..n{
            let is_pos = nums[i] > 0;
            if circular_array_loop(&mut nums, i, is_pos, 0){
                return true;
            }
        }
        false
    }
}

fn circular_array_loop(nums: &mut Vec<i32>, i:usize, is_pos:bool, count:i32) -> bool {
    // println!("{}:{:?}", nums[i], nums);
    if (nums[i] > 0) ^ is_pos{
        return false;
    }
    match nums[i]{
        i32::MAX => false,
        1001 | -1001 => count > 1,
        _ => {
            let n = nums.len();
            let idx = (n + i + nums[i] as usize) % n;
            nums[i] = if is_pos { 1001 } else { -1001 };
            if i == idx{
                nums[i] = i32::MAX;
                return false;
            }
            if circular_array_loop(nums, idx, is_pos, count+1){
                return true;
            }
            nums[i] = i32::MAX;
            false       
        },
    }    
}