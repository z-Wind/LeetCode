// dp(n) = 1 + dp(n-x*x)

use std::collections::HashMap;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp:HashMap<i32,i32> = HashMap::new();
        num_squares(&mut dp, n)
    }
}

fn num_squares(dp:&mut HashMap<i32,i32>, n: i32) -> i32 {
    if n < 4{
        return n;
    }
    if dp.contains_key(&n){
        return *dp.get(&n).unwrap();
    }
    
    let mut ans = i32::MAX;
    let mut i = 1;
    while i*i <= n{
        ans = ans.min(1+num_squares(dp, n-i*i));
        i+=1
    }
    dp.insert(n,ans);
    ans
}