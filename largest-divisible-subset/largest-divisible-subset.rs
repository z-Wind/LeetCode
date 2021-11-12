// dp[i] = if nums[i+1] % nums[i] => nums[i], dp[i+1]

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort();
        
        let mut ans = vec![];
        let mut dp:Vec<Vec<i32>> = vec![vec![];n];
        for i in (0..n).rev(){
            dp[i].push(nums[i]);
            for j in i+1..n{
                let mut temp = vec![nums[i]];
                if nums[j] % nums[i] == 0{
                    temp.extend_from_slice(&dp[j]);
                }
                if dp[i].len() < temp.len(){
                    dp[i] = temp;
                }
            }
            if ans.len() < dp[i].len(){
                ans = dp[i].clone();
            }
        }
        // println!("{:?}", dp);
        ans
    }
}