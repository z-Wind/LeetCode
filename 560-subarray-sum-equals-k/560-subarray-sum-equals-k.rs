// sum[i..j] = sum[0..j] - sum[0..i]

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();

        for i in 1..=n {
            sum[i] = sum[i - 1] + nums[i - 1];
            map.entry(sum[i])
                .and_modify(|x| x.push(i))
                .or_insert(vec![i]);
        }

        let mut ans = 0;
        for i in 0..=n {
            if let Some(v) = map.get(&(sum[i] + k)) {
                match v.binary_search(&i) {
                    Ok(n) => ans += (v.len() - 1 - n) as i32,
                    Err(n) => ans += (v.len() - n) as i32,
                }
            }
        }

        ans
    }
}
