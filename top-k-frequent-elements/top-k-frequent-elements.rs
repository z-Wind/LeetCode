use std::collections::HashMap;
use std::collections::BTreeMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        
        let mut map = HashMap::new();
        for num in nums{
            *map.entry(num).or_insert(0) += 1;
        }
        // println!("{:?}", map);
        
        let mut order_map = BTreeMap::new();
        for (key, val) in map.iter(){
            order_map.entry(val).or_insert(vec![]).push(key);
        }
        // println!("{:?}", order_map);
        
        let mut ans:Vec<i32> = Vec::with_capacity(k);
        let mut i = 0;
        'outer: for (key, vals) in order_map.iter().rev(){
            for &&val in vals{
                ans.push(val);
                i+=1;
                if i == k{
                    break 'outer;
                }
            }
        }
        
        ans
    }
}