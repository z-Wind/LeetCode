impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_buy = (0, i32::MAX);
        let mut max_sell = (0, i32::MIN);
        for i in (0..prices.len()){
            if max_sell.1 < prices[i]{
                max_sell = (i, prices[i]);
                if max_sell.0 > min_buy.0 {
                    profit = profit.max(prices[i]-min_buy.1);
                }
            }
            if min_buy.1 > prices[i]{
                min_buy = (i, prices[i]);
                if i>max_sell.0{
                    max_sell = (0, i32::MIN);
                }
            }
        }
        profit
    }
}