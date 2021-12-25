impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0;nums.len()];
        
        let mut ans = vec![];
        for num in nums{
            let i = num as usize -1;
            counts[i] += 1;
            if counts[i] > 1{
                ans.push(num);
            }
        }
        ans
    }
}