// dp[i] = if nums[i+1] % nums[i] => nums[i], dp[i+1]

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort();
        
        let mut ans = n-1;
        let mut dp:Vec<Vec<i32>> = vec![vec![];n];
        for i in (0..n).rev(){
            dp[i].push(nums[i]);
            let mut max_j = i;
            for j in i+1..n{
                if nums[j] % nums[i] == 0 && (max_j==i || dp[j].len() > dp[max_j].len()){
                    max_j = j;    
                }
            }
            if max_j != i{
                let mut temp = dp[max_j].clone();
                dp[i].append(&mut temp);
            }
            if dp[ans].len() < dp[i].len(){
                ans = i
            }
        }
        // println!("{:?}", dp);
        dp[ans].clone()
    }
}