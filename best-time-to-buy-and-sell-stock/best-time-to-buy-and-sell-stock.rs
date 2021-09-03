impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        max_profit(&prices[..])
    }
}

fn max_profit(prices: &[i32]) -> i32 {
    // println!("{:?}", prices);
    if prices.is_empty() || prices.len() == 1{
        return 0;
    }
    
    let mut profit = 0;
    let mut min_buy = (0,i32::MAX);
    let mut max_sell = (0, i32::MIN);
    for (i,p) in prices.iter().enumerate(){
        if &min_buy.1 > p{
            min_buy = (i, *p);
        } else if &max_sell.1 < p{
            max_sell = (i, *p);
        }
    }
    match max_sell.0 > min_buy.0{
        true => profit = max_sell.1 - min_buy.1,
        false => {
            profit = max_profit(&prices[..=max_sell.0]).max(
                max_profit(&prices[max_sell.0+1..min_buy.0])
            ).max(
                max_profit(&prices[min_buy.0..])
            )
        }
    }
    profit
}