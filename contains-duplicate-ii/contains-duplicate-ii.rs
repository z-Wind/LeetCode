use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map:HashMap<i32,usize> = HashMap::new();
        for (i,&num) in nums.iter().enumerate(){
            match map.insert(num, i){
                None => (),
                Some(j) => {
                    if i-j <= k as usize{
                        return true;
                    }
                },
            }
        }
        false
    }
}