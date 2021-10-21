// https://leetcode.com/problems/coin-change/discuss/778548/C%2B%2B-DP-solution-explained-~100-Time-100-Space

// dp[amount] = min(1+dp[amount-coins[0]], 1+dp[amount-coins[1]], .. , 1+dp[amount-coins[n]])

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let n = amount as usize + 1;
        // creating the base dp array, with first value set to 0
        let mut dp = vec![i32::MAX;n];
        dp[0] = 0;
        // more convenient to have the coins sorted
        coins.sort();
        coins.dedup();
        
        // populating our dp array
        for money in 1..n {
            for &coin in coins.iter() {
                let coin = coin as usize;
                if money < coin {
                    break;
                }
                // if it was a previously not reached cell, we do not add use it
                if dp[money - coin] != i32::MAX{
                    dp[money] = dp[money].min(1 + dp[money - coin]);  
                } 
            }
        }
        if dp[n-1] == i32::MAX{
            -1
        } else {
            dp[n-1]   
        }
    }
}