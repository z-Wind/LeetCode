impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }

        let m = m as usize;
        let n = n as usize;
        let o = max_move as usize + 1;

        let mut dp = vec![vec![vec![0; n]; m]; o];
        for i in 0..m {
            dp[1][i][0] += 1;
        }
        for j in 0..n {
            dp[1][0][j] += 1;
        }
        for i in 0..m {
            dp[1][i][n - 1] += 1;
        }
        for j in 0..n {
            dp[1][m - 1][j] += 1;
        }

        for k in 2..o {
            for i in 0..m {
                for j in 0..n {
                    if i - 1 < m {
                        dp[k][i][j] += dp[k - 1][i - 1][j];
                        dp[k][i][j] %= 1_000_000_000 + 7;
                    }
                    if i + 1 < m {
                        dp[k][i][j] += dp[k - 1][i + 1][j];
                        dp[k][i][j] %= 1_000_000_000 + 7;
                    }
                    if j - 1 < n {
                        dp[k][i][j] += dp[k - 1][i][j - 1];
                        dp[k][i][j] %= 1_000_000_000 + 7;
                    }
                    if j + 1 < n {
                        dp[k][i][j] += dp[k - 1][i][j + 1];
                        dp[k][i][j] %= 1_000_000_000 + 7;
                    }
                }
            }
        }

        //println!("{:?}", dp);
        let mut sum = 0;
        let i = start_row as usize;
        let j = start_column as usize;
        for k in 0..o {
            sum += dp[k][i][j];
            sum %= 1_000_000_000 + 7;
        }
        sum
    }
}
