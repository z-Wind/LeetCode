// https://github.com/z-Wind/LeetCode/blob/main/best-time-to-buy-and-sell-stock-iii/best-time-to-buy-and-sell-stock-iii.rs
// dp[k, i] = max(dp[k, i-1], prices[i] - prices[j] + dp[k-1, j-2]), j=[0..=i-1]
//                  nothing     buy at j, sell at i
use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        MaxProfit_V3(prices.clone())
    }
}

// Version 1
// Time complexity is O(kn^2), space complexity is O(kn)
fn MaxProfitDp_V1(prices: Vec<i32>) -> i32 {
    if (prices.len() <= 1) { return 0 };
    let max_k = if prices.len()%3 == 0 {prices.len()/3} else {prices.len()/3+1};
    let mut dp = vec![vec![0;prices.len()];max_k+1];
    for k in (1..=max_k) {
        for i in (1..prices.len()) {
            let mut min = prices[0].min(prices[1]);
            for j in (2..=i-1){
                min = min.min(prices[j] - dp[k-1][j-2]);
            }
            dp[k][i] = max(dp[k][i-1], prices[i] - min);
        }
    }

    return dp[max_k][prices.len() - 1];
}

// Version 2: Get rid of repeating calculation of min
// because dp[k][i] should be the maximum value
// Time complexity is O(kn), space complexity is O(kn)
pub fn MaxProfitDp_V2(prices: Vec<i32>) -> i32 {
    if (prices.len() <= 1) { return 0 };
    let max_k = if prices.len()%3 == 0 {prices.len()/3} else {prices.len()/3+1};
    let mut dp = vec![vec![0;prices.len()];max_k+1];
    for k in (1..=max_k) {
        dp[k][1] = max(dp[k][0], prices[1] - prices[0]);
        
        let mut min = prices[0].min(prices[1]);
        for i in (2..prices.len()) {
            min = min.min(prices[i] - dp[k-1][i-2]);
            dp[k][i] = max(dp[k][i-1], prices[i] - min);
        }
    }

    return dp[max_k][prices.len() - 1];
}

// Version 3: compact k
//
// dp[k-1][i-2]
// dp[k][i-1]   dp[k][i]
//
// dp[i-2]
// dp[i-1]      dp[i]
//
// Time complexity is O(kn), space complexity is O(n).
pub fn MaxProfit_V3(prices: Vec<i32>) -> i32 {
    if (prices.len() <= 1) { return 0 };
    let max_k = if prices.len()%3 == 0 {prices.len()/3} else {prices.len()/3+1};
    let mut dp = vec![0;prices.len()];
    for k in (1..=max_k) {
        dp[1] = max(dp[0], prices[1] - prices[0]);
        
        let mut min = prices[0].min(prices[1]);
        for i in (2..prices.len()) {
            min = min.min(prices[i] - dp[i-2]);
            dp[i] = max(dp[i-1], prices[i] - min);
        }
    }

    return dp[prices.len() - 1];
}