// https://leetcode.com/problems/top-k-frequent-elements/discuss/81602/Java-O(n)-Solution-Bucket-Sort

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        
        let mut map:HashMap<i32,usize> = HashMap::new();
        for num in nums{
            *map.entry(num).or_insert(0) += 1;
        }
        // println!("{:?}", map);
        
        let mut bucket:Vec<Vec<i32>> = vec![vec![];n+1];
        for (&key, &val) in map.iter(){
            bucket[val].push(key);
        }
        // println!("{:?}", order_map);
        
        let mut ans:Vec<i32> = Vec::with_capacity(k);
        for mut v in bucket.into_iter().rev(){
            ans.append(&mut v);
            if ans.len() >= k{
                break;
            }
        }
        
        ans
    }
}