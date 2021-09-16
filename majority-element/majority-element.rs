use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map:HashMap<i32,i32> = HashMap::new();
        let threshold = nums.len() as i32 / 2;
        for num in nums{
            let entry = map.entry(num).or_insert(0_i32);
            *entry += 1;
            if *entry > threshold{
                return num;
            }
        }
        return 0;
    }
}