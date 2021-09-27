use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len()/3;
        let mut map = HashMap::new();
        let mut ans = Vec::new();
        for num in nums{
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
            if *entry == n+1{
                ans.push(num);
            }
        }
        ans
    }
}