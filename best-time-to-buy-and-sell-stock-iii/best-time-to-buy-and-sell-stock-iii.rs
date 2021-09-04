// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/discuss/135704/Detail-explanation-of-DP-solution
// dp[k, i] = max(dp[k, i-1], prices[i] - prices[j] + dp[k-1, j-1]), j=[0..i-1]
//                  nothing     buy at j, sell at i

use std::cmp::{max};
impl Solution {
    // Version 1
    // Time complexity is O(kn^2), space complexity is O(kn).
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        MaxProfitDp_V6(prices)
    }
    
}

// Version 1
// Time complexity is O(kn^2), space complexity is O(kn)
fn MaxProfitDp_V1(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![vec![0;prices.len()];3];
    for k in (1..=2) {
        for i in (1..prices.len()) {
            let mut min = prices[0];
            for j in (1..=i-1){
                min = min.min(prices[j] - dp[k-1][j-1]);
            }
            dp[k][i] = max(dp[k][i-1], prices[i] - min);
        }
    }

    return dp[2][prices.len() - 1];
}

// Version 2: Get rid of repeating calculation of min
// because dp[k][i] should be the maximum value
// Time complexity is O(kn), space complexity is O(kn)
pub fn MaxProfitDp_V2(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![vec![0;prices.len()];3];
    for k in (1..=2) {
        let mut min = prices[0];
        for i in (1..prices.len()) {
            min = min.min(prices[i] - dp[k-1][i-1]);
            dp[k][i] = max(dp[k][i-1], prices[i] - min);
        }
    }

    return dp[2][prices.len() - 1];
}

// Version 3
// swap the two 'for' loops, save min for each transaction.
pub fn MaxProfitDp_V3(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![vec![0;prices.len()];3];
    let mut min = vec![prices[0];3];
    for i in (1..prices.len()) {
        for k in (1..=2) {
            min[k] = min[k].min(prices[i] - dp[k-1][i-1]);
            dp[k][i] = max(dp[k][i-1], prices[i] - min[k]);
        }
    }

    return dp[2][prices.len() - 1];
}

// Version 4: compact i
//
// dp[k-1][i-1]
// dp[k][i-1]   dp[k][i]
//
// dp[k-1]
// dp[k]   dp[k](update)
//
// Time complexity is O(kn), space complexity is O(k).
pub fn MaxProfit_V4(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![0;3];
    let mut min = vec![prices[0];3];
    for i in (1..prices.len()) {
        for k in (1..=2) {
            min[k] = min[k].min(prices[i] - dp[k-1]);
            dp[k] = max(dp[k], prices[i] - min[k]);
        }
    }

    return dp[2];
}

// Version 5: expand k
pub fn MaxProfitDp_V5(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    let mut dp = vec![0;3];
    let mut min = vec![prices[0];3];
    for i in (1..prices.len()) {
        min[1] = min[1].min(prices[i] - dp[0]);
        dp[1] = max(dp[1], prices[i] - min[1]);

        min[2] = min[2].min(prices[i] - dp[1]);
        dp[2] = max(dp[2], prices[i] - min[2]);
    }

    return dp[2];
}

// Version 6: rename variable and init i from 0
// On every day, we buy the share with the price as low as we can, and sell the share with price as high as we can. For the second transaction, we integrate the profit of first transaction into the cost of the second buy, then the profit of the second sell will be the total profit of two transactions.
pub fn MaxProfitDp_V6(prices: Vec<i32>) -> i32 {
    if (prices.len() == 0) { return 0 };
    
    let mut buy1 = i32::MAX;
    let mut profit1 = 0;
    let mut buy2 = i32::MAX;
    let mut profit2 = 0;
    for i in (0..prices.len()) {
        buy1 = buy1.min(prices[i]);
        profit1 = profit1.max(prices[i] - buy1);

        buy2 = buy2.min(prices[i] - profit1);
        profit2 = profit2.max(prices[i] - buy2);
    }

    profit2
}