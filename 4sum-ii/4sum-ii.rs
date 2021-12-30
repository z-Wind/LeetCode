// https://leetcode.com/problems/4sum-ii/discuss/93920/Clean-java-solution-O(n2)

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let n = nums1.len();
        let mut map = HashMap::new();

        for i in 0..n {
            for j in 0..n {
                let sum = nums1[i] + nums2[j];
                *map.entry(sum).or_insert(0) += 1;
            }
        }

        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                let sum = -(nums3[i] + nums4[j]);
                count += *map.get(&sum).unwrap_or(&0);
            }
        }

        count
    }
}
