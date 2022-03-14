// https://leetcode.com/problems/continuous-subarray-sum/discuss/99499/Java-O(n)-time-O(k)-space

// current one: sum_i = m*k + modk
// previous one: sum_j = n*k + modk
// So we can get
// sum_i - sum_j = (m - n) * k

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        // for from start or start+1 to i
        // [23,2,4,6,6]
        // 7
        
        let mut modk = 0;
        for i in 0..nums.len() {
            modk += nums[i];
            modk %= k;
            if let Some(&prev) = map.get(&modk) {
                // at least two elements
                if i as i32 - prev > 1 {
                    return true;
                }
            } else {
                map.insert(modk, i as i32);
            }
        }
        return false;
    }
}
