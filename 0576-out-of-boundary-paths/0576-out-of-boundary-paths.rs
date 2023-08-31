impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let k = max_move as usize;
        let i = start_row as usize;
        let j = start_column as usize;

        let mut dp = vec![vec![vec![-1; k + 1]; n]; m];
        solve(&mut dp, m, n, i, j, k)
    }
}

fn solve(dp: &mut Vec<Vec<Vec<i32>>>, m: usize, n: usize, i: usize, j: usize, k: usize) -> i32 {
    if i >= m || j >= n {
        return 1;
    }
    if k == 0 {
        return 0;
    }

    if dp[i][j][k] >= 0 {
        return dp[i][j][k];
    }

    // mod 很重要，減少計算量，不然會 Time Limit Exceeded
    let mut sum = solve(dp, m, n, i - 1, j, k - 1);
    sum += solve(dp, m, n, i + 1, j, k - 1);
    sum %= (1_000_000_000 + 7);
    sum += solve(dp, m, n, i, j - 1, k - 1);
    sum %= (1_000_000_000 + 7);
    sum += solve(dp, m, n, i, j + 1, k - 1);
    sum %= (1_000_000_000 + 7);

    dp[i][j][k] = sum;
    return sum;
}
