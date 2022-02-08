// https://leetcode.com/problems/next-greater-element-i/discuss/97595/Java-10-lines-linear-time-complexity-O(n)-with-explanation

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new(); // map from x to next greater element of x
        let mut stack: Vec<i32> = Vec::new();
        for num in nums2 {
            while !stack.is_empty() && *stack.last().unwrap() < num {
                map.insert(stack.pop().unwrap(), num);
            }
            stack.push(num);
        }

        for num in nums1.iter_mut() {
            *num = map.get(num).cloned().unwrap_or(-1);
        }

        return nums1;
    }
}
