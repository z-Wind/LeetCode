use std::collections::HashMap;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_p = 0;
        for w in prices.windows(2){
            let profit = w[1] - w[0];
            if profit > 0{
                max_p += profit;
            }
        }
        
        max_p
    }
}
