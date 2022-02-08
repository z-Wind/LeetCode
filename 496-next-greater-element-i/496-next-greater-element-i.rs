use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums2.len(){
            map.insert(nums2[i], i);
        }
        
        let mut ans = Vec::new();
        'outer: for num in nums1 {
            if let Some(start) = map.get(&num) {
                for i in start+1..nums2.len() {
                    if nums2[i] > num {
                        ans.push(nums2[i]);
                        continue 'outer;
                    }
                }
                ans.push(-1);
            }
        }
        
        ans
    }
}