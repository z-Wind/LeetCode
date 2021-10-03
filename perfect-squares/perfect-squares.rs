// dp(n) = 1 + dp(n-x*x)

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        if n < 4{
            return n;
        }
        let n = n as usize;
        let mut dp:Vec<i32> = vec![i32::MAX; n +1];
        dp[0] = 0;
        for i in (1..=n){
            let mut j = 1;
            while j*j <= i{
                dp[i] = dp[i].min(1+dp[i-j*j]);
                j+=1;
            }
        }
        dp[n]
    }
}
