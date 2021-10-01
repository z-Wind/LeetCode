use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::new();
        for num in nums{
            match set.contains(&num){
                true => set.remove(&num),
                false => set.insert(num),
            };
        }
        set.into_iter().collect()
    }
}