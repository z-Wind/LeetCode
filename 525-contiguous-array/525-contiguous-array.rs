// https://leetcode.com/problems/contiguous-array/discuss/1743637/C%2B%2B-with-Explanation-oror-Easy-to-Understand-oror-Unordered_Map

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        mp.insert(0, -1);

        let mut sum = 0;
        let mut longest_subarray = 0;
        for i in 0..nums.len() as i32 {
            match nums[i as usize] {
                0 => sum -= 1,
                1 => sum += 1,
                x => panic!("{} is not zero or one", x),
            }
            if let Some(idx) = mp.get(&sum) {
                if longest_subarray < i - idx {
                    longest_subarray = i - idx;
                }
            } else {
                mp.insert(sum, i);
            }
        }
        
        longest_subarray as i32
    }
}
