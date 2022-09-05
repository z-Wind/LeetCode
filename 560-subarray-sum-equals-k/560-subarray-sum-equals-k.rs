// https://leetcode.com/problems/subarray-sum-equals-k/discuss/1760146/C%2B%2B-Easy-solution-prefix-sum-map-easy-understanding-hashmap
// sum[i..j] = sum[0..j] - sum[0..i]

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut sum = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(&freq) = map.get(&(sum - k)) {
                ans += freq;
            }
            map.entry(sum).and_modify(|x| *x += 1).or_insert(1);
        }
        ans
    }
}
