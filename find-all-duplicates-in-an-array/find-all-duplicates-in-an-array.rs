use std::collections::HashSet;
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {        
        let n = nums.len();
        let mut ans = HashSet::new();
        for i in 0..n{            
            while nums[i] != i as i32 + 1{
                // println!("{}:{:?} {:?}", i, nums, ans);
                let j = nums[i] as usize -1;
                if nums[i] == nums[j]{
                    ans.insert(nums[i]);
                    break;
                }
                nums.swap(i,j);
            }            
        }
        ans.into_iter().collect()
    }
}