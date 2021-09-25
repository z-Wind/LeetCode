// find side length
// dp(i,j)=min(dp(i−1,j), dp(i,j−1), dp(i−1,j−1)) + 1.
//
// dp(i-1,j-1) dp(i-1,j)
// dp(i,j-1)   dp(i,j)

use std::cmp::min;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp:Vec<Vec<i32>> = vec![vec![0;n];m];
        let mut max_side = 0;
        for i in (0..m){
            match matrix[i][0]{
                '0' => dp[i][0] = 0,
                '1' => dp[i][0] = 1,
                _ => panic!(),
            }
            max_side = max_side.max(dp[i][0]);
        }
        for j in (0..n){
            match matrix[0][j]{
                '0' => dp[0][j] = 0,
                '1' => dp[0][j] = 1,
                _ => panic!(),
            }
            max_side = max_side.max(dp[0][j]);
        }
        
        for i in (1..m){
            for j in (1..n){
                match matrix[i][j]{
                    '1' => dp[i][j] = min(dp[i-1][j],min(dp[i][j-1],dp[i-1][j-1])) + 1,
                    '0' => dp[i][j] = 0,
                    _ => panic!(),
                }
                max_side = max_side.max(dp[i][j]);
            }
        }
        max_side * max_side
    }
}