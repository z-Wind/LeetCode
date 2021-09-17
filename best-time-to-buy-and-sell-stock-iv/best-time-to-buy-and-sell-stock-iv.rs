// https://github.com/z-Wind/LeetCode/blob/main/best-time-to-buy-and-sell-stock-iii/best-time-to-buy-and-sell-stock-iii.rs
// dp[k, i] = max(dp[k, i-1], prices[i] - prices[j] + dp[k-1, j-1]), j=[0..i-1]
//                  nothing  sell at i,   buy at j, 

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 { return 0 };
        
        let mut k_n = k as usize;
        //if k_n >= n/2, then you can make maximum number of transactions.
        if k_n >=  n/2 {
            let mut maxPro = 0;
            for i in (1..n) {
                if prices[i] > prices[i-1] {
                    maxPro += prices[i] - prices[i-1];
                }
            }
            return maxPro;
        }
        
        let mut dp = vec![0;k_n+1];
        let mut min = vec![prices[0];k_n+1];
        for i in (1..n) {
            for k in (1..=k_n) {
                min[k] = min[k].min(prices[i] - dp[k-1]);
                dp[k] = dp[k].max(prices[i] - min[k]);
            }
        }

        return dp[k_n];
    }
}