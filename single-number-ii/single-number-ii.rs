use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map:HashMap<i32,i32> = HashMap::new();
        for num in nums{
            *map.entry(num).or_insert(0) += 1;
            if map.get(&num).unwrap() == &3{
                map.remove(&num);
            }
        }
        *map.keys().next().unwrap()
    }
}