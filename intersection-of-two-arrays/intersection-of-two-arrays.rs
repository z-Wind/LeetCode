use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1:HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        let set2:HashSet<i32> = HashSet::from_iter(nums2.into_iter());
        set1.intersection(&set2).cloned().collect()
    }
}