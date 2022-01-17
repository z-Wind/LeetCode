// https://leetcode.com/problems/ones-and-zeroes/discuss/95814/c%2B%2B-DP-solution-with-comments

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut memo = vec![vec![0; n + 1]; m + 1];
        for s in strs {
            // count number of zeroes and ones in current string
            let (numZeroes, numOnes) = s.chars().fold((0, 0), |acc, c| {
                if c == '0' {
                    (acc.0 + 1, acc.1)
                } else {
                    (acc.0, acc.1 + 1)
                }
            });
            // println!("{}:({},{})", s, numZeroes, numOnes);

            // memo[i][j] = the max number of strings that can be formed with i 0's and j 1's
            // from the first few strings up to the current string s
            // Catch: have to go from bottom right to top left
            // Why? If a cell in the memo is updated(because s is selected),
            // we should be adding 1 to memo[i][j] from the previous iteration (when we were not considering s)
            // If we go from top left to bottom right, we would be using results from this iteration => overcounting
            for i in (numZeroes..=m).rev() {
                for j in (numOnes..=n).rev() {
                    memo[i][j] = memo[i][j].max(memo[i - numZeroes][j - numOnes] + 1);
                }
            }
        }
        return memo[m][n];
    }
}