use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result = 0;
        for (min, count) in map.iter() {
            let max = min + 1;
            match map.get(&max) {
                None => (),
                Some(c) => result = result.max(count + c),
            }
        }

        result
    }
}