use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map:HashMap<i32,usize> = HashMap::new();
        for i in (0..nums.len()) {
            match map.insert(nums[i], i){
                Some(j) => {
                    if i-j <= k as usize{
                        return true;
                    }
                },
                None => (),
            }
        }
        false
    }
}