impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        let n  = n as usize;
        // dp(n) = dp(n-1) + dp(n-2)
        let mut step:Vec<i32> = vec![1;n+1];
        for i in (1..=n){
            if i > 1{
                step[i] = step[i-1] + step[i-2]
            } else {
                step[i] = step[i-1]
            }
        }
        step[n]
    }
}