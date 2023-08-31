// dp[k][i][j] = dp[k-1][i-1][j] + dp[k-1][i+1][j] + dp[k-1][i][j-1] + dp[k-1][i][j+1]

const M:i32 = 1_000_000_000 + 7;
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }

        let m = m as usize;
        let n = n as usize;
        let x = start_row as usize;
        let y = start_column as usize;
        let mut count = 0;

        let mut prev = vec![vec![0; n]; m];
        for i in 0..m {
            prev[i][0] += 1;
        }
        for j in 0..n {
            prev[0][j] += 1;
        }
        for i in 0..m {
            prev[i][n - 1] += 1;
        }
        for j in 0..n {
            prev[m - 1][j] += 1;
        }
        count += prev[x][y];

        for k in 2..=max_move as usize {
            let mut cur = vec![vec![0; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if i != 0 {
                        cur[i][j] += prev[i - 1][j];
                        cur[i][j] %= M;
                    }
                    if i != m - 1 {
                        cur[i][j] += prev[i + 1][j];
                        cur[i][j] %= M;
                    }
                    if j != 0 {
                        cur[i][j] += prev[i][j - 1];
                        cur[i][j] %= M;
                    }
                    if j != n - 1 {
                        cur[i][j] += prev[i][j + 1];
                        cur[i][j] %= M;
                    }
                }
            }
            prev = cur;
            count += prev[x][y];
            count %= M;
        }

        count
    }
}