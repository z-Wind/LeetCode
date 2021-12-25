// https://leetcode.com/problems/find-all-duplicates-in-an-array/discuss/92387/Java-Simple-Solution

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {        
        let n = nums.len();
        let mut ans = vec![];
        for i in 0..n{
            let idx = nums[i].abs() as usize -1;
            if nums[idx] < 0{
                ans.push(idx as i32 + 1);
            } else {
                nums[idx] = -nums[idx];
            }
        }
        // println!("{:?}", nums);
        ans
    }
}