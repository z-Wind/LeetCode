use std::collections::BTreeSet;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let mut bts: BTreeSet<i32> = BTreeSet::new();
        for i in 0..nums.len() {
            if i > k {
                bts.remove(&nums[i - 1 - k]);
            }
            if bts
                .range(nums[i].saturating_sub(t)..=nums[i].saturating_add(t))
                .next()
                .is_some()
            {
                return true;
            }
            bts.insert(nums[i]);
        }
        false
    }
}