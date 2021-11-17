// dp[i][pos] = max(if nums[i] < nums[j] 1+dp[j][neg]) for j in i+1..n
// dp[i][neg] = max(if nums[i] > nums[j] 1+dp[j][pos]) for j in i+1..n

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp:Vec<Vec<i32>> = vec![vec![1,1];n];
        let mut ans = 0;
        for i in (0..n-1).rev(){
            for j in (i+1..n).rev(){
                if nums[i] < nums[j]{
                    dp[i][0] = dp[i][0].max(1+dp[j][1]) 
                } else if nums[i] > nums[j]{
                    dp[i][1] = dp[i][1].max(1+dp[j][0]) 
                }
            }
        }
        // println!("{:?}", dp);
        dp[0].iter().max().cloned().unwrap()
    }
}
