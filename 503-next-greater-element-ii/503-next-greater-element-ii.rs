// https://github.com/z-Wind/LeetCode/blob/main/496-next-greater-element-i/496-next-greater-element-i.rs

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut map = HashMap::new();
        let mut stack = Vec::new();
        for &num in nums.iter().cycle().take(n * 2) {
            while !stack.is_empty() && *stack.last().unwrap() < num {
                map.entry(stack.pop().unwrap()).or_insert(VecDeque::new()).push_back(num);
            }
            stack.push(num);
        }
        // println!("{:?}", map);
        
        
        let mut ans = Vec::with_capacity(n);
        for num in nums.iter() {
            match map.get_mut(num) {
                None => ans.push(-1),
                Some(v) => ans.push(v.pop_front().unwrap()),
            }
        }
        ans
    }
}