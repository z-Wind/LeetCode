// https://github.com/z-Wind/LeetCode/blob/main/best-time-to-buy-and-sell-stock-iii/best-time-to-buy-and-sell-stock-iii.rs
// dp[k, i] = max(dp[k, i-1], prices[i] - prices[j] + dp[k-1, j-1]), j=[0..i-1]
//                  nothing  sell at i,   buy at j, 

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        max_profit(k as usize, prices)
    }
}

fn max_profit(k_n: usize, prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![0;k_n+1];
    let mut min = vec![prices[0];k_n+1];
    for i in (1..prices.len()) {
        for k in (1..=k_n) {
            min[k] = min[k].min(prices[i] - dp[k-1]);
            dp[k] = dp[k].max(prices[i] - min[k]);
        }
    }

    return dp[k_n];
}