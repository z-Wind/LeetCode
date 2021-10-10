// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/discuss/75928/Share-my-DP-solution-(By-State-Machine-Thinking)
//
// three state
// s0: stay or cooldown from s2
// s1: stay or buy from s0
// s2: sell from s1
//
// three lists mean total money at i
// s0[i] = max(s0[i - 1], s2[i - 1]); // Stay at s0, or rest from s2
// s1[i] = max(s1[i - 1], s0[i - 1] - prices[i]); // Stay at s1, or buy from s0
// s2[i] = s1[i - 1] + prices[i]; // Only one way from s1

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n <= 1 { return 0; }
		let mut s0 = vec![0;n];
		let mut s1 = vec![0;n];
		let mut s2 = vec![0;n];
		s1[0] = -prices[0];
		s0[0] = 0;
		s2[0] = i32::MIN;
		for i in 1..n {
			s0[i] = max(s0[i - 1], s2[i - 1]);
			s1[i] = max(s1[i - 1], s0[i - 1] - prices[i]);
			s2[i] = s1[i - 1] + prices[i];
		}
		return max(s0[n - 1], s2[n - 1]);
    }
}
