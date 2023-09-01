impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![-1; word2.len()]; word1.len()];

        solve(&mut dp, 0, 0, word1, word2)
    }
}

fn solve(dp: &mut Vec<Vec<i32>>, i: usize, j: usize, word1: &[u8], word2: &[u8]) -> i32 {
    if i >= word1.len() && j >= word2.len() {
        return 0;
    } else if i >= word1.len() {
        return (word2.len() - j) as i32;
    } else if j >= word2.len() {
        return (word1.len() - i) as i32;
    } else if dp[i][j] != -1 {
        return dp[i][j];
    }

    let step = if word1[i] == word2[j] {
        solve(dp, i + 1, j + 1, word1, word2)
    } else {
        (1 + solve(dp, i, j + 1, word1, word2))
            .min((1 + solve(dp, i + 1, j, word1, word2)))
            .min((2 + solve(dp, i + 1, j + 1, word1, word2)))
    };

    dp[i][j] = step;
    dp[i][j]
}
