impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_buy = i32::MAX;
        for i in (0..prices.len()){
            if min_buy > prices[i]{
                min_buy = prices[i];
            } else if prices[i] - min_buy > max_profit{
                max_profit = prices[i] - min_buy;
            }
        }
        max_profit
    }
}