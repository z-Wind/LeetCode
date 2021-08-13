const INF: i32 = i32::MAX;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if !(1 <= nums[i] && nums[i] <= nums.len() as i32) {
                nums[i] = INF;
            }
        }

        for i in 0..nums.len() {
            let index = nums[i].abs() - 1;
            if index != INF - 1 {
                nums[index as usize] = -nums[index as usize].abs();
            }
        }
        //println!("{:?}",nums);
        for i in 0..nums.len() {
            if nums[i].is_positive() {
                return i as i32 + 1;
            }
        }

        nums.len() as i32 + 1
    }
}