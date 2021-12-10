// https://leetcode.com/problems/partition-equal-subset-sum/discuss/90592/01-knapsack-detailed-explanation

// dp[i][j] means whether the specific sum j can be gotten from the first i numbers
// nums[i-1] means ith num
// dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i-1]]

// dp[i-1][j-nums[i-1]] dp[i-1][j] 
//                      dp[i][j]

// should reverse update
// dp[j-nums[i-1]] dp[j]
//                 dp[j](update)

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().fold(0, |acc, x| acc+x);
        // println!("{}", sum);
        // sum % 2 == 1
        if sum & 1 == 1{
            return false;
        }
        sum >>= 1;
        
        // can_partition_1(nums, sum as usize)
        can_partition_2(nums, sum as usize)
    }
}

fn can_partition_1 (nums: Vec<i32>, sum: usize) -> bool {
    let n = nums.len();
    let mut dp = vec![vec![false;sum+1];n+1];    
    dp[0][0] = true;
    
    for i in 1..n+1 {
        dp[i][0] = true;
    }
    // for j in 1..sum+1 {
    //     dp[0][j] = false;
    // }
    
    for i in 1..n+1 {
        for j in 1..sum+1 {
            dp[i][j] = dp[i-1][j];
            let num = nums[i-1] as usize;
            if j >= num{
                dp[i][j] = (dp[i][j] || dp[i-1][j-num]);
            }
        }
    }
   
    dp[n][sum]
}

fn can_partition_2 (nums: Vec<i32>, sum: usize) -> bool {
    let n = nums.len();
    let mut dp = vec![false;sum+1];    
    dp[0] = true;
    
    for i in 1..n+1 {
        for j in (1..sum+1).rev() {
            let num = nums[i-1] as usize;
            if j >= num{
                dp[j] = (dp[j] || dp[j-num]);
            }
        }
    }
   
    dp[sum]
}