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

    let step1 = match word1[i..].iter().position(|c| *c == word2[j]) {
        Some(step) => step as i32 + solve(dp, i + step + 1, j + 1, word1, word2),
        None => 1 + solve(dp, i + 1, j, word1, word2),
    };
    let step2 = match word2[j..].iter().position(|c| *c == word1[i]) {
        Some(step) => step as i32 + solve(dp, i + 1, j + step + 1, word1, word2),
        None => 1 + solve(dp, i, j + 1, word1, word2),
    };
    let step3 = 2 + solve(dp, i + 1, j + 1, word1, word2);

    // println!("{},{}: {}, {}, {}",i,j,step1,step2,step3);

    dp[i][j] = step1.min(step2).min(step3);
    dp[i][j]
}
