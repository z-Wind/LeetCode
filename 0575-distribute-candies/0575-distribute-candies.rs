use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let set:HashSet<i32> = candy_type.into_iter().collect();
        set.len().min(n/2) as i32
    }
}