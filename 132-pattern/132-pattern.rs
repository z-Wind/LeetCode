use std::collections::BTreeMap;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 2{
            return false;
        }
        
        let mut min = vec![i32::MAX;n];
        let mut max = vec![i32::MIN;n];
        let mut map = BTreeMap::new();
        map.insert(nums[0], 0);
        for k in 1..n{
            min[k] = nums[k-1].min(min[k-1]);
            max[k] = nums[k-1].max(max[k-1]);
            if min[k] < nums[k] && nums[k] < max[k]{
                for (_, &j) in map.range(nums[k]+1..){
                    if min[j] < nums[k]{
                        return true;
                    }
                }
            }
            map.insert(nums[k], k);
        }
        false
    }
}