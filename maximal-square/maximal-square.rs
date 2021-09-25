// find side length
// dp(i,j)=min(dp(i−1,j), dp(i,j−1), dp(i−1,j−1)) + 1.
//
// dp(i-1,j-1) dp(i-1,j)
// dp(i,j-1)   dp(i,j)
//
// pre     dp(j)
// dp(j-1) dp(j)(update)
//
use std::cmp::min;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp:Vec<i32> = vec![0;n];
        let mut max_side = 0;
        for j in (0..n){
            dp[j] = match matrix[0][j]{
                '0' => 0,
                '1' => 1,
                _ => panic!(),
            };
            max_side = max_side.max(dp[j]);
        }
        // println!("{:?}",dp);
        
        for i in (1..m){
            let mut pre = dp[0];
            dp[0] = match matrix[i][0]{
                '0' => 0,
                '1' => 1,
                _ => panic!(),
            };
            max_side = max_side.max(dp[0]);
            for j in (1..n){
                let temp = dp[j];
                dp[j] = match matrix[i][j]{
                    '1' => min(dp[j],min(dp[j-1],pre)) + 1,
                    '0' => 0,
                    _ => panic!(),
                };
                pre = temp;
                max_side = max_side.max(dp[j]);
            }
            // println!("{:?}",dp);
        }
        max_side * max_side
    }
}