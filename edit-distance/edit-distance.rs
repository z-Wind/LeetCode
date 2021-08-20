// https://leetcode.com/problems/edit-distance/discuss/25846/C%2B%2B-O(n)-space-DP
//  if word1[i - 1] == word2[j - 1]
//      dp[i][j] = dp[i - 1][j - 1]
//elif word1[i - 1] != word2[j - 1], get minimum
//      1. Replace dp[i][j] = dp[i - 1][j - 1] + 1
//      2  Delete  dp[i][j] = dp[i - 1][j] + 1
//      3. Insert  dp[i][j] = dp[i][j - 1] + 1.

// space: 2d -> 1d
// dp[i-1][j-1],  dp[i-1][j]
// dp[i][j-1],    dp[i][j]
// The corresponding code:
// dp[i][j] = min(dp[i - 1][j - 1] , dp[i][j - 1] , dp[i - 1][j] ) + 1

// When using 1d array. We are updating cur[ ] from beginning to the end again and again (Row of 2D array times). The relative position is like this:

// pre,       cur[i]
// cur[i-1]   cur[i](waiting for the update)
// The corresponding code:
// cur[i] = min(pre, cur[i], cur[i - 1]) + 1
use std::cmp::min;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let mut pre:usize; // [i-1, j-1]
        let mut cur = vec![0;n + 1];
        for j in (1..=n) {
            cur[j] = j; //[0, j] insert
        }
        for i in (1..=m) {
            pre = cur[0];
            cur[0] = i; // [i, 0] delete
            for j in (1..=n) {
                let temp = cur[j];
                if (word1[i - 1..i] == word2[j - 1..j]) {
                    cur[j] = pre;
                } else {
                    //       min(  replace,    insert,   delete  )
                    //       min( [i-1][j-1], [i][j-1], [i-1][j] )
                    cur[j] = min(   pre, min(cur[j - 1], cur[j])) + 1;
                }
                pre = temp;
            }
        }
        return cur[n] as i32;
    }
}