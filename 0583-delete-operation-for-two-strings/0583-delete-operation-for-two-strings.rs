// https://leetcode.com/problems/delete-operation-for-two-strings/solutions/1195726/c-python-java-short-easy-solutions-w-explanation-optimization-from-brute-force-to-dp/

// dp[i][j] will denote the number of steps required to equalize word1[:i] (characters from index 0 till i) and word2[:j] (characters from index 0 till j).

// dp[i-1][j-1] dp[i-1][j]
// dp[i][j-1]   dp[i][j]

// dp[j-1]_prev dp[j]_prev
// dp[j-1]      dp[j]

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut word1 = word1.as_bytes();
        let mut word2 = word2.as_bytes();
        if word2.len() > word1.len() {
            std::mem::swap(&mut word1, &mut word2);
        }

        let n = word1.len();
        let m = word2.len();
        let mut dp = vec![vec![0_i32; m + 1]; 2];

        for i in 0..=n {
            for j in 0..=m {
                dp[i & 1][j] = if i == 0 || j == 0 {
                    (i + j) as i32
                } else {
                    if word1[i - 1] == word2[j - 1] {
                        dp[(i - 1) & 1][j - 1]
                    } else {
                        (1 + dp[(i - 1) & 1][j]).min(1 + dp[i & 1][j - 1])
                    }
                };
            }
        }

        dp[n & 1][m]
    }
}
