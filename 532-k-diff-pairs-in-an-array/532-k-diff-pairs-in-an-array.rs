// https://leetcode.com/problems/k-diff-pairs-in-an-array/discuss/1756967/Well-Explained-oror-Two-Easy-Solutions

use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut count = 0;
        for (num, freq) in map.iter() {
            if (k > 0 && map.contains_key(&(num + k))) || (k == 0 && *freq > 1) {
                count += 1;
            }
        }

        return count;
    }
}
