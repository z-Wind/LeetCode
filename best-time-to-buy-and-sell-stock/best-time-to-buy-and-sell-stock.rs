impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_buy = i32::MAX;
        for price in prices{
            if min_buy > price{
                min_buy = price;
            } else if price - min_buy > max_profit{
                max_profit = price - min_buy;
            }
        }
        max_profit
    }
}