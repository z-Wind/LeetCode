use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map:HashMap<i32,usize> = HashMap::new();
        for (i,num) in nums.into_iter().enumerate(){
            match map.get_mut(&num){
                None => {
                    map.insert(num, i);
                },
                Some(j) => {
                    if i-*j <= k as usize{
                        return true;
                    }
                    *j = i;
                },
            }
        }
        false
    }
}