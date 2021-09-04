
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut tt = prices.clone();
        tt.sort();
        if tt == prices{
            return prices.last().unwrap() - prices[0];
        }
        
        let mut max_p = 0;
        let mut i = 0;
        while i < prices.len(){
            let t1 = max_profit(&prices,0,i);
            let t2 = max_profit(&prices,i,prices.len());
            let sum1 = t1.1+t2.1;
            
            i = t2.0;
            if i == prices.len(){
                break;
            }
            let t1 = max_profit(&prices,0,i);
            let t2 = max_profit(&prices,i,prices.len());
            let sum2 = t1.1+t2.1;
            
            max_p = max_p.max(sum1).max(sum2);
            // println!("{} {:?},{:?} => {}", i, &prices[0..i], &prices[i..], max_p);
            i = t2.0+1;
        }
        max_p
    }
}

fn max_profit(prices: &Vec<i32>, start:usize, end:usize) -> (usize, i32) {
    let mut max_profit = (prices.len(),0);
    let mut min_buy = i32::MAX;
    let mut min_i = start+1;
    for i in (start..end){
        if min_buy > prices[i]{
            min_buy = prices[i];
            min_i = i;
        } else if prices[i] - min_buy > max_profit.1{
            max_profit = (min_i, prices[i] - min_buy);
        }
    }
    max_profit
}