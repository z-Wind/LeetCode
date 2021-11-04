//  2 => 1x1
//  3 => 1x2
//  4 => 2x2
//  5 => 3x2
//  6 => 3x3
//  7 => 3x2x2
//  8 => 3x3x2
//  9 => 3x3x3
// 10 => 3x3x2x2
// 11 => 3x3x3x2
// 12 => 3x3x3x3
// 13 => 3x3x3x2x2
// 14 => 3x3x3x3x2
// 15 => 3x3x3x3x3
// 16 => 3x3x3x3x2x2
// 17 => 3x3x3x3x3x2
// 18 => 3x3x3x3x3x3
// 19 => 3x3x3x3x3x2x2
// 20 => 3x3x3x3x3x3x2

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1;7.max(n+1)];
        dp[2] = 1;
        dp[3] = 2;
        dp[4] = 4;
        dp[5] = 6;
        dp[6] = 9;
        let mut prefix = 3;
        for i in 7..=n{
            match (i-4) % 3{
                0 => dp[i] = prefix * 2 * 2,
                1 => dp[i] = prefix * 3 * 2,
                2 => {
                    dp[i] = prefix * 3 * 3;
                    prefix *= 3;
                },
                _ => panic!("impossible"),
            }
        }
        
        dp[n]
    }
}