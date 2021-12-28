impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let idx = (nums[i].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        }
        // println!("{:?}", nums);
        
        nums.iter()
            .enumerate()
            .filter(|(_, x)| **x > 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}
